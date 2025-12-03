use anyhow::Result;

fn get_joltage(batteries: Vec<usize>) -> usize {
    let (idx_max, max) = batteries
        .iter()
        .take(batteries.len() - 1)
        .enumerate()
        .rev() //Make sure we get the first occurence of max and not the last
        .max_by_key(|&(_, v)| v)
        .unwrap();

    let second = *batteries.iter().skip(idx_max + 1).max().unwrap();
    10 * max + second
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(get_joltage)
        .sum()
}

fn part1(input: &str) -> String {
    solve(input).to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("../../../inputs/03.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../inputs/03-example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "357");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "TODO");
    }
}
