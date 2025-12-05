use std::{ops::RangeInclusive, usize};

use anyhow::Result;

fn parse_ranges(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .lines()
        .take_while(|s| !s.trim().is_empty())
        .map(|s| {
            let (s1, s2) = s.split_once("-").unwrap();
            (
                s1.parse().unwrap(),
                s2.parse().unwrap(),
            )
        })
        .map(|(a, b)| a..=b)
        .collect()
}

fn parse_ids(input: &str) -> Vec<usize> {
    input.lines().filter_map(|s| s.parse().ok()).collect()
}

fn solve(input: &str) -> usize {
    let ranges = parse_ranges(input);
    let ids = parse_ids(input);
    ids.into_iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn part1(input: &str) -> String {
    solve(input).to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("../../../inputs/05.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../inputs/05-example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "3");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "TODO");
    }
}
