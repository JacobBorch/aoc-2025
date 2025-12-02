use anyhow::Result;

fn parse(line: &str) -> (&str, &str) {
    let mut nums = line.split("-");
    (nums.next().unwrap(), nums.next().unwrap())
}

fn is_invalid_part1(id: &str) -> bool {
    if id.len() % 2 != 0 {
        return false;
    }
    let mid = id.len() / 2;
    let a = &id[..mid];
    let b = &id[mid..];
    a == b
}

fn is_invalid_part2(id: &str) -> bool {
    let len = id.len();
    for i in 1..=(len / 2) {
        if len % i != 0 {
            continue;
        }
        let slice = &id[..i];
        let mut found = false;
        for j in 1..(len / i) {
            let start = j * i;
            let end = start + i;
            let other = &id[start..end];

            if slice != other {
                found = true;
                break;
            }
        }
        if !found {
            return true;
        }
    }
    false
}

fn solve<F>(input: &str, f: F) -> usize
where
    F: Fn(&str) -> bool,
{
    let pairs: Vec<(&str, &str)> = input.split(",").map(parse).collect();
    let mut total: usize = 0;
    for (a, b) in pairs {
        let mut from = a.parse::<usize>().unwrap();
        let to = b.parse::<usize>().unwrap();
        while from <= to {
            let s = from.to_string();
            if f(&s) {
                total += from;
            }
            from += 1;
        }
    }
    total
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
