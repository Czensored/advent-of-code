use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = parse_input(input);

    for _ in 0..25 {
        blink(&mut input);
    }

    Some(input.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(run(input, 75))
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn blink(input: &mut Vec<u64>) {
    let mut new_input = Vec::with_capacity(input.len() * 2);

    for &n in input.iter() {
        if n == 0 {
            new_input.push(1);
        } else if let Some((left, right)) = split_u64_middle(n) {
            new_input.push(left);
            new_input.push(right);
        } else {
            new_input.push(n * 2024);
        }
    }

    *input = new_input;
}

fn split_u64_middle(n: u64) -> Option<(u64, u64)> {
    let s = n.to_string();
    let len = s.len();

    if len % 2 == 0 {
        let mid = len / 2;
        let (left, right) = s.split_at(mid);
        let left_num = left.parse::<u64>().ok()?;
        let right_num = right.parse::<u64>().ok()?;
        Some((left_num, right_num))
    } else {
        None
    }
}

fn run(input: &str, blinks: usize) -> u64 {
    let mut counts: HashMap<u64, u64> = HashMap::new();

    // Initialize counts from input
    for n in parse_input(input) {
        *counts.entry(n).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut new_counts: HashMap<u64, u64> = HashMap::new();

        for (&n, &count) in &counts {
            let next = blink_number(n);
            for value in next {
                *new_counts.entry(value).or_insert(0) += count;
            }
        }

        counts = new_counts;
    }

    // Sum total stones
    counts.values().sum()
}

fn blink_number(n: u64) -> Vec<u64> {
    if n == 0 {
        vec![1]
    } else if has_even_digits(n) {
        split_u64_middle_two(n)
    } else {
        vec![n * 2024]
    }
}

fn has_even_digits(n: u64) -> bool {
    let digits = if n == 0 {
        1
    } else {
        ((n as f64).log10().floor() as u64 + 1) as usize
    };

    digits % 2 == 0
}

fn split_u64_middle_two(n: u64) -> Vec<u64> {
    let s = n.to_string();
    let mid = s.len() / 2;
    let (left, right) = s.split_at(mid);

    let left_num = left.parse::<u64>().unwrap();
    let right_num = right.parse::<u64>().unwrap();

    vec![left_num, right_num]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
