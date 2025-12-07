use std::ops::{Add, Mul};

use anyhow::Result;
use aoc_lib::Grid;

fn parse_numbers(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .take_while(|l| !l.contains(['*', '+']))
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect()
}

fn parse_operators(input: &str) -> Vec<char> {
    input
        .lines()
        .last()
        .unwrap()
        .replace(" ", "")
        .chars()
        .collect()
}

fn solve(input: &str) -> usize {
    let numbers = parse_numbers(input);
    let operators = parse_operators(input);
    println!("{:?}", numbers);
    println!("{:?}", operators);

    let mut sum = 0;

    for (x, &op) in operators.iter().enumerate() {
        let col_iter = numbers
            .iter()
            .filter_map(move |row: &Vec<usize>| row.get(x).copied());

        let reduce_func = if op == '*' { Mul::mul } else { Add::add };
        sum += col_iter.reduce(reduce_func).unwrap();
    }

    sum
}

fn part1(input: &str) -> String {
    solve(input).to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("../../../inputs/06.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../inputs/06-example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "4277556");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "TODO");
    }
}
