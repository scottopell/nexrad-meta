use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDate, NaiveTime};
use clap::{Parser, Subcommand};
use nexrad_data::archive::Identifier;
use nexrad_data::aws::archive::{download_file, list_files};
use reqwest::Client;
use serde_json::Value;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "-rust/", env!("CARGO_PKG_VERSION"),);

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Aws(AwsArgs),
    Inspect(InspectArgs),
    Stations(StationsArgs),
}

#[derive(Parser)]
struct AwsArgs {
    /// Site identifier (e.g., KDMX)
    #[arg(default_value = "KDMX")]
    site: String,

    /// Date in YYYY-MM-DD format
    #[arg(default_value = "2022-03-05")]
    date: String,

    /// Start time in HH:MM format
    #[arg(default_value = "23:30")]
    start_time: String,

    /// Stop time in HH:MM format
    #[arg(default_value = "23:30")]
    stop_time: String,
}

#[derive(Parser)]
struct InspectArgs {
    /// File to inspect
    file: String,
}

#[derive(Parser)]
struct StationsArgs {
    /// List all stations
    #[arg(long)]
    list: bool,

    /// Station identifier (e.g., KOKX)
    station: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Aws(args) => handle_aws_command(args).await,
        Commands::Inspect(args) => handle_inspect_command(args),
        Commands::Stations(args) => handle_stations_command(args).await,
    }
}

async fn handle_aws_command(args: AwsArgs) -> Result<()> {
    let site = &args.site;
    let date = NaiveDate::parse_from_str(&args.date, "%Y-%m-%d")
        .context("Failed to parse date. Please use YYYY-MM-DD format.")?;
    let start_time = NaiveTime::parse_from_str(&args.start_time, "%H:%M")
        .context("Failed to parse start time. Please use HH:MM format.")?;
    let stop_time = NaiveTime::parse_from_str(&args.stop_time, "%H:%M")
        .context("Failed to parse stop time. Please use HH:MM format.")?;

    println!("Listing files for {} on {}...", site, date);
    let file_ids = list_files(site, &date)
        .await
        .context("Failed to list files from AWS")?;

    if file_ids.is_empty() {
        println!("No files found for the specified date/site to download.");
        return Ok(());
    }

    println!("Found {} files.", file_ids.len());

    let start_index = get_nearest_file_index(&file_ids, start_time);
    let stop_index = get_nearest_file_index(&file_ids, stop_time);

    println!("Downloading {} files...", stop_index - start_index + 1);

    for file_id in file_ids
        .iter()
        .skip(start_index)
        .take(stop_index - start_index + 1)
    {
        download_and_save_file(file_id).await?;
    }

    println!("Downloaded {} files.", stop_index - start_index + 1);

    Ok(())
}

fn handle_inspect_command(args: InspectArgs) -> Result<()> {
    println!("Inspecting file: {}", args.file);
    // TODO: Implement file inspection logic
    Ok(())
}

async fn handle_stations_command(args: StationsArgs) -> Result<()> {
    let client = Client::builder().user_agent(APP_USER_AGENT).build()?;
    let response = client
        .get("https://api.weather.gov/radar/stations")
        .send()
        .await
        .context("Failed to fetch stations data")?;

    if let Err(e) = response.error_for_status_ref() {
        return Err(e.into());
    } else if !response.status().is_success() {
        return Err(anyhow!(
            "weather.gov response failed with {}",
            response.status()
        ));
    }
    let data: Value = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    if args.list {
        println!("Listing all stations:");
        if let Some(features) = data["features"].as_array() {
            for feature in features {
                if let (Some(id), Some(name)) = (
                    feature["properties"]["id"].as_str(),
                    feature["properties"]["name"].as_str(),
                ) {
                    println!("{}: {}", id, name);
                }
            }
        }
    } else if let Some(station) = args.station {
        if let Some(features) = data["features"].as_array() {
            if let Some(station_data) = features
                .iter()
                .find(|f| f["properties"]["id"].as_str() == Some(&station))
            {
                println!("Current state of station: {}", station);
                if let Some(props) = station_data["properties"].as_object() {
                    for (key, value) in props {
                        println!("  {}: {}", key, value);
                    }
                }
            } else {
                println!("Station {} not found.", station);
            }
        }
    } else {
        println!("Please specify --list or provide a station identifier.");
    }

    Ok(())
}

async fn download_and_save_file(file_id: &Identifier) -> Result<()> {
    println!("Downloading file \"{}\"...", file_id.name());
    let file = download_file(file_id.clone())
        .await
        .context("Failed to download file from AWS")?;

    println!("Data file size (bytes): {}", file.data().len());

    if !Path::new("downloads").exists() {
        println!("Creating downloads directory...");
        create_dir("downloads").context("Failed to create downloads directory")?;
    }

    println!("Writing file to disk as: {}", file_id.name());
    let mut downloaded_file =
        File::create(format!("downloads/{}", file_id.name())).context("Failed to create file")?;
    downloaded_file
        .write_all(file.data().as_slice())
        .context("Failed to write file contents")?;

    Ok(())
}

fn get_nearest_file_index(files: &[Identifier], target_time: NaiveTime) -> usize {
    files
        .iter()
        .enumerate()
        .min_by_key(|(_, file)| {
            file.date_time()
                .expect("file has valid date time")
                .time()
                .signed_duration_since(target_time)
                .num_seconds()
                .abs()
        })
        .map(|(index, _)| index)
        .unwrap_or(0)
}
