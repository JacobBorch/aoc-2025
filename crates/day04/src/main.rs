use anyhow::Result;
use aoc_lib::Grid;

fn solve(input: &str) -> usize {
    let graph = Grid::from(input);
    graph
        .iter()
        .filter(|&(_, c)| c == '@')
        .filter(|&(pos, c)| graph.neighbors(pos).filter(|&c| c == '@').count() < 4)
        .count()
}

fn part1(input: &str) -> String {
    solve(input).to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("../../../inputs/04.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../inputs/04-example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "13");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "TODO");
    }
}
