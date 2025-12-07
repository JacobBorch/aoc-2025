use anyhow::Result;

fn solve(input: &str) -> usize {
    let mut splits = 0;

    let grid_length = input.lines().next().unwrap().len();
    let mut old_beams = vec![false; grid_length];

    for line in input.lines() {
        let mut new_beams = vec![false; grid_length];
        for (i, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    if !old_beams[i] {
                        continue;
                    }
                    new_beams[i - 1] = true;
                    new_beams[i + 1] = true;
                    splits += 1;
                }
                '.' => new_beams[i] = old_beams[i] || new_beams[i],
                'S' => new_beams[i] = true,
                _ => unreachable!(),
            }
        }
        old_beams = new_beams;
    }

    splits
}

fn part1(input: &str) -> String {
    solve(input).to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

fn main() -> Result<()> {
    let input = include_str!("../../../inputs/07.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../inputs/07-example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "21");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "TODO");
    }
}
