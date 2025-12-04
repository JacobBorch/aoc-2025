use anyhow::Result;
use aoc_lib::{Grid, Position};

fn find_removable_rolls(grid: &Grid) -> Vec<(Position, char)> {
    grid.iter()
        .filter(|&(_, c)| c == '@')
        .filter(|&(pos, _)| grid.neighbors(pos).filter(|&c| c == '@').count() < 4)
        .collect()
}

fn solve(input: &str) -> usize {
    let grid = Grid::from(input);
    find_removable_rolls(&grid).len()
}

fn solve_part2(input: &str) -> usize {
    let mut grid = Grid::from(input);

    std::iter::from_fn(|| {
        let removable = find_removable_rolls(&grid);
        if removable.is_empty() {
            return None;
        }
        for (pos, _) in &removable {
            grid.grid[pos.y as usize][pos.x as usize] = '.';
        }
        Some(removable.len())
    })
    .sum()
}

fn part1(input: &str) -> String {
    solve(input).to_string()
}

fn part2(input: &str) -> String {
    solve_part2(input).to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../inputs/04.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../inputs/04-example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "13");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "43");
    }
}
