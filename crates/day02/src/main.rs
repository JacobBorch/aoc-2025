use anyhow::Result;

fn parse(line: &str) -> (&str, &str) {
    let mut nums = line.split("-");
    (nums.next().unwrap(), nums.next().unwrap())
}

fn is_invalid_part1(id: &str) -> bool {
    let middle = id.len() / 2;
    id.len() % 2 == 0 && id[..middle] == id[middle..]
}

fn is_invalid_part2(id: &str) -> bool {
    let bytes = id.as_bytes();
    let len = bytes.len();

    (1..=len / 2).any(|i| len % i == 0 && bytes.chunks(i).all(|chunk| chunk == &bytes[..i]))
}

fn solve<F>(input: &str, f: F) -> usize
where
    F: Fn(&str) -> bool,
{
    input
        .split(',')
        .map(parse)
        .flat_map(|(start, end)| {
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            start..=end
        })
        .filter(|&num| f(&num.to_string()))
        .sum()
}

fn part1(input: &str) -> String {
    solve(input, is_invalid_part1).to_string()
}

fn part2(input: &str) -> String {
    solve(input, is_invalid_part2).to_string()
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
        assert_eq!(part2(EXAMPLE), "4174379265");
    }
}
