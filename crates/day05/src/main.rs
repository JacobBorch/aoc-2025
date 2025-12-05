use std::{ops::RangeInclusive, usize};

use anyhow::Result;

fn parse_ranges(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .lines()
        .take_while(|s| !s.trim().is_empty())
        .map(|s| {
            let (s1, s2) = s.split_once("-").unwrap();
            (s1.parse().unwrap(), s2.parse().unwrap())
        })
        .map(|(a, b)| a..=b)
        .collect()
}

fn parse_ids(input: &str) -> Vec<usize> {
    input.lines().filter_map(|s| s.parse().ok()).collect()
}

fn merge_ranges(mut input_ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let mut output: Vec<RangeInclusive<usize>> = Vec::new();
    input_ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let mut current_opt = None;
    for range in &input_ranges {
        if current_opt.is_none() {
            current_opt = Some((range.start(), range.end()));
            continue;
        }
        let current = current_opt.unwrap();
        if range.start() <= current.1 {
            current_opt = Some((current.0.min(range.start()), current.1.max(range.end())));
            continue;
        }
        output.push(*current.0..=*current.1);
        current_opt = Some((range.start(), range.end()));
    }
    if let Some((&a, &b)) = current_opt {
        output.push(a..=b);
    }
    output
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
    let ranges = parse_ranges(input);
    merge_ranges(ranges)
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<usize>()
        .to_string()
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
        assert_eq!(part2(EXAMPLE), "14");
    }
}
