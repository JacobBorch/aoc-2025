use std::{fs, path::PathBuf};

use anyhow::Context;

pub fn input_path(day: u8) -> PathBuf {
    PathBuf::from(format!("inputs/{:02}.txt", day))
}

pub fn read_input(day: u8) -> anyhow::Result<String> {
    let path = input_path(day);
    fs::read_to_string(&path)
        .with_context(|| format!("failed to read AoC input at {}", path.display()))
}
