use std::collections::HashSet;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u64> {
    let mut rm = parse_input(input);

    rm.step(100);

    Some(rm.calculate_safety_factor())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rm = parse_input(input);

    for t in 0..=10000 {
        if rm.has_vertical_line(10) {
            // println!("Iteration: {}", t);
            // rm.print();
            return Some(t);
        }

        rm.step(1);
    }

    None
}

fn parse_input(input: &str) -> Room {
    // Check for test case
    let mut rm = if input.lines().count() == 12 {
        Room {
            dimensions: (11, 7),
            robots: vec![],
        }
    } else {
        Room {
            dimensions: (101, 103),
            robots: vec![],
        }
    };

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let cleaned: String = line
            .chars()
            .map(|c| {
                if c.is_ascii_digit() || c == '-' {
                    c
                } else {
                    ' '
                }
            })
            .collect();

        let nums: Vec<i64> = cleaned
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        if nums.len() == 4 {
            rm.robots.push(Robot {
                position: (nums[0] as u64, nums[1] as u64),
                velocity: (nums[2], nums[3]),
            });
        }
    }

    rm
}

struct Robot {
    position: (u64, u64),
    velocity: (i64, i64),
}

struct Room {
    dimensions: (u64, u64),
    robots: Vec<Robot>,
}

impl Room {
    fn step(&mut self, n: usize) {
        for _ in 0..n {
            let (w, h) = self.dimensions;
            for r in &mut self.robots {
                let nx = (r.position.0 as i128 + r.velocity.0 as i128).rem_euclid(w as i128) as u64;
                let ny = (r.position.1 as i128 + r.velocity.1 as i128).rem_euclid(h as i128) as u64;

                r.position = (nx, ny);
            }
        }
    }

    fn calculate_safety_factor(&self) -> u64 {
        let (w, h) = self.dimensions;
        let midx = w / 2;
        let midy = h / 2;

        let mut q1 = 0u64;
        let mut q2 = 0u64;
        let mut q3 = 0u64;
        let mut q4 = 0u64;

        for r in &self.robots {
            let (x, y) = r.position;

            if x < midx && y < midy {
                q1 += 1;
            } else if x > midx && y < midy {
                q2 += 1;
            } else if x < midx && y > midy {
                q3 += 1;
            } else if x > midx && y > midy {
                q4 += 1;
            }
        }

        q1 * q2 * q3 * q4
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (w, h) = self.dimensions;
        let mut grid = vec![vec![' '; w as usize]; h as usize];

        for r in &self.robots {
            let (x, y) = r.position;
            if x < w && y < h {
                grid[y as usize][x as usize] = '#';
            }
        }

        for row in grid {
            let line: String = row.into_iter().collect();
            println!("{}", line);
        }
    }

    fn has_vertical_line(&self, len: usize) -> bool {
        if len == 0 {
            return true;
        }

        let len_u64 = len as u64;
        let (_, h) = self.dimensions;

        // O(1) membership checks
        let positions: HashSet<(u64, u64)> = self.robots.iter().map(|r| r.position).collect();

        positions.iter().any(|&(x, y)| {
            // fits without wrapping and all cells below are present
            y + len_u64 - 1 < h && (1..len_u64).all(|dy| positions.contains(&(x, y + dy)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
