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
    FetchInput {
        day: u8,
    },

    NewDay {
        day: u8,

        #[arg(long)]
        no_fetch: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::FetchInput { day } => fetch_input(cli.year, day)?,
        Commands::NewDay { day, no_fetch } => new_day(cli.year, day, no_fetch)?,
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

fn new_day(year: i32, day: u8, no_fetch: bool) -> Result<()> {
    if !(1..=25).contains(&day) {
        bail!("Day must be between 1 and 25, got {day}");
    }

    let day_pad = format!("{day:02}");
    let crate_name = format!("day{day_pad}");

    let root = PathBuf::from(".");
    let crates_dir = root.join("crates");
    let day_crate_dir = crates_dir.join(&crate_name);
    let inputs_dir = root.join("inputs");

    if day_crate_dir.exists() {
        println!(
            "Crate {} already exists at {}",
            crate_name,
            day_crate_dir.display()
        );
    } else {
        println!(
            "Creating crate {} in {}",
            crate_name,
            day_crate_dir.display()
        );
        fs::create_dir_all(day_crate_dir.join("src")).context("creating day crate directory")?;

        let cargo_toml = format!(
            r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2024"

[dependencies]
aoc-lib = {{ path = "../aoc-lib" }}
anyhow = {{ workspace = true }}
"#,
            crate_name = crate_name,
        );

        fs::write(day_crate_dir.join("Cargo.toml"), cargo_toml)
            .context("writing day Cargo.toml")?;

        let main_rs = format!(
            r#"use anyhow::Result;

fn part1(input: &str) -> String {{
    todo!()
}}

fn part2(input: &str) -> String {{
    todo!()
}}

fn main() -> Result<()> {{
    let input = include_str!("../../../inputs/{day_pad}.txt");
    println!("Part 1: {{}}", part1(&input));
    println!("Part 2: {{}}", part2(&input));
    Ok(())
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const EXAMPLE: &str = include_str!("../../../inputs/{day_pad}-example.txt");

    #[test]
    fn part1_example() {{
        assert_eq!(part1(EXAMPLE), "TODO");
    }}

    #[test]
    fn part2_example() {{
        assert_eq!(part2(EXAMPLE), "TODO");
    }}
}}
"#,
            day_pad = day_pad,
        );

        fs::write(day_crate_dir.join("src/main.rs"), main_rs).context("writing day main.rs")?;
    }

    fs::create_dir_all(&inputs_dir).context("creating inputs directory")?;

    let real_input = inputs_dir.join(format!("{day_pad}.txt"));
    let example_input = inputs_dir.join(format!("{day_pad}-example.txt"));

    if !example_input.exists() {
        fs::write(&example_input, b"").context("creating example input file")?;
        println!("Created {}", example_input.display());
    }

    if !real_input.exists() {
        if no_fetch {
            fs::write(&real_input, b"").context("creating empty real input file")?;
            println!("Created empty {}", real_input.display());
        } else {
            fetch_input(year, day)?;
        }
    } else {
        println!("Real input already exists at {}", real_input.display());
    }

    let workspace_toml_path = root.join("Cargo.toml");
    let mut workspace_toml =
        fs::read_to_string(&workspace_toml_path).context("reading workspace Cargo.toml")?;

    let member_entry = format!(r#""crates/{crate_name}""#);

    if !workspace_toml.contains(&member_entry) {
        println!("Adding {} to workspace members", member_entry);

        let needle = "members = [";
        if let Some(idx) = workspace_toml.find(needle) {
            let start = idx + needle.len();
            if let Some(end) = workspace_toml[start..].find(']') {
                let insert_pos = start + end;
                let to_insert = format!(",\n    \"crates/{crate_name}\"");
                workspace_toml.insert_str(insert_pos, &to_insert);
            } else {
                bail!("Could not find closing ] for workspace members");
            }
        } else {
            bail!("Could not find `members = [` in workspace Cargo.toml");
        }

        fs::write(&workspace_toml_path, workspace_toml)
            .context("writing updated workspace Cargo.toml")?;
    } else {
        println!("Workspace already contains {}", member_entry);
    }

    println!("Scaffolded day {day} ({crate_name})");
    Ok(())
}
