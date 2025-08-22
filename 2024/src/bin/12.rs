use std::collections::HashMap;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut area = HashMap::new();

    for line in input.iter() {
        for &item in line.iter() {
            *area.entry(item).or_insert(0) += 1;
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(1930));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
