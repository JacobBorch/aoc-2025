use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "AoC helper tool")]
struct Cli {
    #[arg(long, global = true, default_value_t = 2025)]
    year: i32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    FetchInput { day: u8 },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::FetchInput { day } => fetch_input(cli.year, day)?,
    }

    Ok(())
}

fn fetch_input(year: i32, day: u8) -> Result<()> {
    if !(1..=25).contains(&day) {
        bail!("Day must be between 1 and 25, got {day}");
    }

    let session = env::var("AOC_SESSION").context("AOC_SESSION environment variable not set")?;

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let day_pad = format!("{day:02}");
    let out_dir = PathBuf::from("inputs");
    let out_file = out_dir.join(format!("{day_pad}.txt"));

    fs::create_dir_all(&out_dir).context("creating inputs directory")?;

    println!("Fetching {url} -> {}", out_file.display());

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&url)
        .header("Cookie", format!("session={session}"))
        .send()
        .context("sending request")?;

    if !resp.status().is_success() {
        bail!("HTTP error: {}", resp.status());
    }

    let body = resp.text().context("reading response body")?;
    fs::write(&out_file, body).context("writing input file")?;

    println!("Wrote {}", out_file.display());
    Ok(())
}
