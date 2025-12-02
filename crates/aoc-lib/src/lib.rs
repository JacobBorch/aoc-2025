use std::path::PathBuf;

pub fn input_path(day: u8) -> PathBuf {
    PathBuf::from(format!("inputs/{:02}.txt", day))
}

pub fn modulo(a: i32, m: i32) -> i32 {
    ((a % m) + m) % m
}
