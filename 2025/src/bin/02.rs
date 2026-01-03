use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    for (start, end) in input {
        let mut seen = HashSet::new();
        for num_digits in digits_u64(start)..=digits_u64(end) {
            if num_digits % 2 != 0 {
                continue;
            }

            let chunk_length = num_digits / 2;

            for n in numbers_with_len(chunk_length) {
                let x = repeat_number(n, 2);
                if (start..=end).contains(&x) {
                    seen.insert(x);
                }
            }
        }
        sum += seen.iter().sum::<u64>();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    for (start, end) in input {
        let mut seen = HashSet::new();
        for num_digits in digits_u64(start)..=digits_u64(end) {
            for chunk_length in 1..=num_digits / 2 {
                if num_digits % chunk_length != 0 {
                    continue;
                }

                let num_chunks = num_digits / chunk_length;

                for n in numbers_with_len(chunk_length) {
                    let check_number = repeat_number(n, num_chunks);
                    if (start..=end).contains(&check_number) {
                        seen.insert(check_number);
                    }
                }
            }
        }
        sum += seen.iter().sum::<u64>();
    }

    Some(sum)
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|pair| pair.trim())
        .map(|pair| {
            let (a, b) = pair.split_once('-').expect("missing '-'");
            let start = a.trim().parse::<u64>().expect("bad start");
            let end = b.trim().parse::<u64>().expect("bad end");
            (start, end)
        })
        .collect()
}

fn digits_u64(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

fn numbers_with_len(len: u32) -> impl Iterator<Item = u64> {
    let start = 10u64.pow(len - 1);
    let end = 10u64.pow(len);
    start..end
}

fn repeat_number(n: u64, times: u32) -> u64 {
    let len = digits_u64(n);
    let mut result = 0u64;
    let pow = 10u64.pow(len);

    for _ in 0..times {
        result = result * pow + n;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
