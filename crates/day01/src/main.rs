use std::iter;

use anyhow::Result;

fn parse_cmd(input: &str) -> (char, i32) {
    let mut chars = input.chars();
    let letter = chars.next().unwrap();
    let digits: String = chars.collect();
    let number = digits.parse::<i32>().unwrap();
    (letter, number)
}

fn solve<F, I>(input: &str, generator: F) -> usize
where
    F: Fn((char, i32)) -> I,
    I: Iterator<Item = i32>,
{
    input
        .lines()
        .map(parse_cmd)
        .flat_map(generator)
        .scan(50, |pos, step| {
            *pos = aoc_lib::modulo(*pos + step, 100);
            Some(*pos)
        })
        .filter(|&pos| pos == 0)
        .count()
}

fn part1(input: &str) -> String {
    solve(input, |(dir, num)| {
        let val = if dir == 'R' { num } else { -num };
        iter::once(val)
    })
    .to_string()
}

fn part2(input: &str) -> String {
    solve(input, |(dir, num)| {
        let val = if dir == 'R' { 1 } else { -1 };
        iter::repeat(val).take(num as usize)
    })
    .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../inputs/01.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../inputs/01-example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "3");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "6");
    }
}
