use std::ops::{Add, Mul};

use anyhow::Result;
use aoc_lib::{Grid, transpose};

fn parse_numbers_part1(input: &str) -> Vec<Vec<usize>> {
    let lines = input
        .lines()
        .take_while(|l| !l.contains(['*', '+']))
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect();
    transpose(lines)
}

fn parse_numbers_part2(input: &str) -> Vec<Vec<usize>> {
    let grid = Grid::from(input);
    let mut output = Vec::new();

    let mut tmp = Vec::new();
    for x in 0..grid.grid[0].len() {
        let col_iter = grid.column(x).unwrap();
        let num = col_iter
            .skip_while(|c| !c.is_numeric())
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse();

        if num.is_err() {
            output.push(tmp);
            tmp = Vec::new();
            continue;
        }

        tmp.push(num.unwrap());
    }
    output.push(tmp);

    output
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

fn solve(input: &str, numbers: Vec<Vec<usize>>) -> usize {
    let operators = parse_operators(input);
    numbers
        .into_iter()
        .zip(operators.into_iter())
        .filter_map(|(row, op)| {
            let reduce_func = if op == '*' { Mul::mul } else { Add::add };
            row.into_iter().reduce(reduce_func)
        })
        .sum::<usize>()
}

fn part1(input: &str) -> String {
    solve(input, parse_numbers_part1(input)).to_string()
}

fn part2(input: &str) -> String {
    solve(input, parse_numbers_part2(input)).to_string()
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
        assert_eq!(part2(EXAMPLE), "3263827");
    }
}
