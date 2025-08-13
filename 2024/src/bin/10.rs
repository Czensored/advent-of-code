use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == 0 {
                let mut seen_map = HashSet::new();
                let trailhead_sum = trailhead_goes_to_nine(row, col, &input, &mut seen_map);
                sum += trailhead_sum;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == 0 {
                let trailhead_sum = trailhead_rating(row, col, &input);
                sum += trailhead_sum;
            }
        }
    }

    Some(sum)
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|num| num.to_digit(10))
                .map(|d| d as u64)
                .collect()
        })
        .collect()
}

fn trailhead_goes_to_nine(
    row: usize,
    col: usize,
    grid: &[Vec<u64>],
    seen_map: &mut HashSet<(usize, usize)>,
) -> u64 {
    if seen_map.contains(&(row, col)) {
        return 0;
    }
    if grid[row][col] == 9 {
        seen_map.insert((row, col));
        return 1;
    }

    let mut sum = 0;

    if row != 0 && grid[row - 1][col] == grid[row][col] + 1 {
        sum += trailhead_goes_to_nine(row - 1, col, grid, seen_map);
    }
    if col != 0 && grid[row][col - 1] == grid[row][col] + 1 {
        sum += trailhead_goes_to_nine(row, col - 1, grid, seen_map);
    }
    if row < grid.len() - 1 && grid[row + 1][col] == grid[row][col] + 1 {
        sum += trailhead_goes_to_nine(row + 1, col, grid, seen_map);
    }
    if col < grid[0].len() - 1 && grid[row][col + 1] == grid[row][col] + 1 {
        sum += trailhead_goes_to_nine(row, col + 1, grid, seen_map);
    }

    sum
}

fn trailhead_rating(row: usize, col: usize, grid: &[Vec<u64>]) -> u64 {
    if grid[row][col] == 9 {
        return 1;
    }

    let mut sum = 0;

    if row != 0 && grid[row - 1][col] == grid[row][col] + 1 {
        sum += trailhead_rating(row - 1, col, grid);
    }
    if col != 0 && grid[row][col - 1] == grid[row][col] + 1 {
        sum += trailhead_rating(row, col - 1, grid);
    }
    if row < grid.len() - 1 && grid[row + 1][col] == grid[row][col] + 1 {
        sum += trailhead_rating(row + 1, col, grid);
    }
    if col < grid[0].len() - 1 && grid[row][col + 1] == grid[row][col] + 1 {
        sum += trailhead_rating(row, col + 1, grid);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
