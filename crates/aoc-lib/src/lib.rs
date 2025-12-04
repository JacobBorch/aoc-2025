use std::path::PathBuf;

pub fn input_path(day: u8) -> PathBuf {
    PathBuf::from(format!("inputs/{:02}.txt", day))
}

pub fn modulo(a: i32, m: i32) -> i32 {
    ((a % m) + m) % m
}

pub struct Grid {
    pub grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn iter(&self) -> impl Iterator<Item = (Position, char)> + '_ {
        self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| ((x, y).into(), c))
        })
    }

    pub fn neighbors<T: Into<Position>>(&self, pos: T) -> impl Iterator<Item = char> + '_ {
        let position = pos.into();
        OFFSETS.iter().filter_map(move |o| {
            let nx = position.x + o.x;
            let ny = position.y + o.y;
            self.get((nx, ny))
        })
    }

    pub fn get<T: Into<Position>>(&self, t: T) -> Option<char> {
        let pos = t.into();
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        self.grid
            .get(pos.y as usize)?
            .get(pos.x as usize)
            .map(|c| *c)
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self {
            grid: value.lines().map(|l| l.chars().collect()).collect(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

pub const fn pos(x: isize, y: isize) -> Position {
    Position { x, y }
}

pub const OFFSETS: [Position; 8] = [
    pos(-1, -1),
    pos(-1, 0),
    pos(-1, 1),
    pos(0, -1),
    pos(0, 1),
    pos(1, -1),
    pos(1, 0),
    pos(1, 1),
];
