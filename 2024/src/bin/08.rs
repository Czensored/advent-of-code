use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let positions = get_antenna_positions(&grid);

    let mut antinode_pos = HashSet::new();

    for (_, pos_list) in &positions {
        if pos_list.len() < 2 {
            continue;
        }

        for i in 0..pos_list.len() {
            for j in (i + 1)..pos_list.len() {
                let (x1, y1) = pos_list[i];
                let (x2, y2) = pos_list[j];

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                // Back-project from (x1, y1) using (-dx, -dy)
                let bx = x1 as isize - dx;
                let by = y1 as isize - dy;
                if bx >= 0 && bx < rows as isize && by >= 0 && by < cols as isize {
                    antinode_pos.insert((bx as usize, by as usize));
                }

                // Forward-project from (x2, y2) using (+dx, +dy)
                let fx = x2 as isize + dx;
                let fy = y2 as isize + dy;
                if fx >= 0 && fx < rows as isize && fy >= 0 && fy < cols as isize {
                    antinode_pos.insert((fx as usize, fy as usize));
                }
            }
        }
    }

    Some(antinode_pos.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let positions = get_antenna_positions(&grid);

    let mut antinode_pos = HashSet::new();

    for (_, pos_list) in &positions {
        if pos_list.len() < 2 {
            continue;
        }

        for &(x, y) in pos_list {
            antinode_pos.insert((x, y));
        }

        for i in 0..pos_list.len() {
            for j in (i + 1)..pos_list.len() {
                let (x1, y1) = pos_list[i];
                let (x2, y2) = pos_list[j];

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                // Back-project from (x1, y1) using (-dx, -dy)
                let mut bx = x1 as isize - dx;
                let mut by = y1 as isize - dy;

                while bx >= 0 && bx < rows as isize && by >= 0 && by < cols as isize {
                    antinode_pos.insert((bx as usize, by as usize));
                    bx -= dx;
                    by -= dy;
                }

                // Forward-project from (x2, y2) using (+dx, +dy)
                let mut fx = x2 as isize + dx;
                let mut fy = y2 as isize + dy;
                while fx >= 0 && fx < rows as isize && fy >= 0 && fy < cols as isize {
                    antinode_pos.insert((fx as usize, fy as usize));
                    fx += dx;
                    fy += dy;
                }
            }
        }
    }

    Some(antinode_pos.len() as u64)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_antenna_positions(grid: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut positions = HashMap::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch != '.' {
                positions.entry(ch).or_insert_with(Vec::new).push((row, col));
            }
        }
    }
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
