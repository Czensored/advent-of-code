advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let mut neighbors = 0;

            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;

                    if nr >= 0
                        && nc >= 0
                        && (nr as usize) < rows
                        && (nc as usize) < cols
                        && grid[nr as usize][nc as usize] == '@'
                    {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    let mut count = 0;
    let mut num_removed = 1;

    while num_removed != 0 {
        num_removed = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' {
                    continue;
                }

                let mut neighbors = 0;

                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;

                        if nr >= 0
                            && nc >= 0
                            && (nr as usize) < rows
                            && (nc as usize) < cols
                            && grid[nr as usize][nc as usize] == '@'
                        {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    count += 1;
                    num_removed += 1;
                    grid[r][c] = '.';
                }
            }
        }
    }

    Some(count)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
