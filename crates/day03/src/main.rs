use anyhow::Result;

fn get_joltage(batteries: Vec<usize>, num_length: usize) -> usize {
    if num_length == 1 {
        return batteries.into_iter().max().unwrap();
    }
    let (idx_max, max) = batteries
        .iter()
        .take(batteries.len() - num_length + 1)
        .enumerate()
        .rev()
        .max_by_key(|&(_, v)| v)
        .unwrap();

    10usize.pow((num_length - 1) as u32) * max
        + get_joltage(
            batteries.into_iter().skip(idx_max + 1).collect(),
            num_length - 1,
        )
}

fn solve<F: Fn(Vec<usize>) -> usize>(input: &str, f: F) -> usize {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(f)
        .sum()
}

fn part1(input: &str) -> String {
    solve(input, |vec| get_joltage(vec, 2)).to_string()
}

fn part2(input: &str) -> String {
    solve(input, |vec| get_joltage(vec, 12)).to_string()
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
        assert_eq!(part2(EXAMPLE), "3121910778619");
    }
}
