use anyhow::Result;

fn solve(input: &str) -> usize {
    todo!()
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
