advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = parse_input(input);
    let mut sum = 0;

    for ingredient in &ingredients {
        for (a, b) in &ranges {
            if (a..=b).contains(&ingredient) {
                sum += 1;
                break;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse_input(input);
    let mut sum = 0;

    ranges.sort_unstable();

    let mut cur_start = ranges[0].0;
    let mut cur_end = ranges[0].1;

    for &(start, end) in &ranges[1..] {
        if start <= cur_end + 1 {
            cur_end = cur_end.max(end);
        } else {
            sum += cur_end - cur_start + 1;
            cur_start = start;
            cur_end = end;
        }
    }

    sum += cur_end - cur_start + 1;

    Some(sum)
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut parts = input.split("\n\n");

    let ranges_part = parts.next().expect("missing ranges section");

    let numbers_part = parts.next().expect("missing numbers section");

    let ranges = ranges_part
        .lines()
        .map(|line| {
            let mut it = line.split('-');

            let start: u64 = it
                .next()
                .expect("missing range start")
                .parse()
                .expect("invalid range start");

            let end: u64 = it
                .next()
                .expect("missing range end")
                .parse()
                .expect("invalid range end");

            (start, end)
        })
        .collect();

    let numbers = numbers_part
        .lines()
        .map(|line| line.trim().parse::<u64>().expect("invalid number"))
        .collect();

    (ranges, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
