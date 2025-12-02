use anyhow::Result;
use aoc_lib::read_input;

fn parse_cmd(input: &str) -> (char, u32) {
    let mut chars = input.chars();
    let letter = chars.next().unwrap();
    let digits: String = chars.collect();
    let number = digits.parse::<u32>().unwrap();
    (letter, number)
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(parse_cmd)
        .map(|(dir, num)| {
            if dir == 'R' {
                num as i32
            } else {
                -(num as i32)
            }
        })
        .scan(50, |pos, move_amount| {
            *pos = aoc_lib::modulo(*pos + move_amount, 100);
            Some(*pos)
        })
        .filter(|&pos| pos == 0)
        .count()
}

fn solve2(input: &str) -> u32 {
    let pairs: Vec<(char, u32)> = input.lines().map(parse_cmd).collect();
    let mut times = 0;
    let mut position: i32 = 50;
    for (direction, mut number) in pairs {
        while number > 0 {
            match direction {
                'L' => {
                    position -= 1;
                }
                'R' => {
                    position += 1;
                }
                _ => unreachable!(),
            }
            position = aoc_lib::modulo(position, 100);
            if position == 0 {
                times += 1;
            }
            number -= 1;
        }
    }
    times
}

fn part1(input: &str) -> String {
    solve(input).to_string()
}

fn part2(input: &str) -> String {
    solve2(input).to_string()
}

fn main() -> Result<()> {
    let input = read_input(1)?;
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
