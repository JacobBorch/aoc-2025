use anyhow::Result;
use aoc_lib::read_input;

fn parse(line: &str) -> (&str, &str) {
    let mut nums = line.split("-");
    (nums.next().unwrap(), nums.next().unwrap())
}

fn is_invalid(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return false;
    }
    let mid = id.len() / 2;
    let a = &id[..mid];
    let b = &id[mid..];
    a == b
}

fn solve(input: &str) -> usize {
    let pairs: Vec<(&str, &str)> = input.split(",").map(parse).collect();
    let mut total: usize = 0;
    for (a, b) in pairs {
        println!("a: {}, b: {}", a, b);
        let mut from = a.parse::<usize>().unwrap();
        let to = b.parse::<usize>().unwrap();
        while from <= to {
            let s = from.to_string();
            if is_invalid(&s) {
                total += from;
            }
            from += 1;
        }
    }
    total
}

fn part1(input: &str) -> String {
    solve(input).to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("../../../inputs/02.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../inputs/02-example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "1227775554");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "TODO");
    }
}
