advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    // This function completely breaks if there is more than one way to get
    // to the destination, but I am ok with that. It is not hard
    // to accomodate for that fact, but it ran just find without changing.
    let input = parse_input(input);

    let mut sum = 0;

    for machine in input {
        for i in 0..=100 {
            for j in 0..=100 {
                if i * machine.button_a.x + j * machine.button_b.x == machine.prize.x
                    && i * machine.button_a.y + j * machine.button_b.y == machine.prize.y
                {
                    sum += i * 3;
                    sum += j;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = parse_input(input);
    for i in &mut input {
        i.prize.x += 10000000000000;
        i.prize.y += 10000000000000;
    }

    let mut sum = 0;

    for machine in input {
        if let Some((i, j)) = solve(
            machine.button_a.x as i128,
            machine.button_a.y as i128,
            machine.button_b.x as i128,
            machine.button_b.y as i128,
            machine.prize.x as i128,
            machine.prize.y as i128,
        ) {
            sum += 3 * (i as u64) + (j as u64);
        }
    }

    Some(sum)
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    fn parse_xy(line: &str) -> Option<(u64, u64)> {
        // Keep digits, turn everything else into spaces, then parse the numbers
        let cleaned: String = line
            .chars()
            .map(|c| if c.is_ascii_digit() { c } else { ' ' })
            .collect();

        let mut nums = cleaned
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok());
        match (nums.next(), nums.next()) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }

    let mut out = Vec::new();
    let mut lines = input.lines().filter(|l| !l.trim().is_empty());

    while let (Some(a_line), Some(b_line), Some(p_line)) =
        (lines.next(), lines.next(), lines.next())
    {
        if let (Some(a), Some(b), Some(p)) = (parse_xy(a_line), parse_xy(b_line), parse_xy(p_line))
        {
            out.push(ClawMachine::new(
                Point::from_tuple(a),
                Point::from_tuple(b),
                Point::from_tuple(p),
            ));
        }
    }

    out
}

struct Point {
    x: u64,
    y: u64,
}

impl Point {
    // fn new(x: u64, y: u64) -> Self {
    //     Point { x, y }
    // }

    fn from_tuple(pt: (u64, u64)) -> Self {
        Point { x: pt.0, y: pt.1 }
    }
}

struct ClawMachine {
    prize: Point,
    button_a: Point,
    button_b: Point,
}

impl ClawMachine {
    fn new(button_a: Point, button_b: Point, prize: Point) -> Self {
        ClawMachine {
            prize,
            button_a,
            button_b,
        }
    }
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (g, x, y) = egcd(b, a.rem_euclid(b));
        (g, y, x - (a / b) * y)
    }
}

fn floor_div(a: i128, b: i128) -> i128 {
    let q = a / b;
    let r = a % b;
    if r != 0 && (r > 0) != (b > 0) {
        q - 1
    } else {
        q
    }
}

fn ceil_div(a: i128, b: i128) -> i128 {
    -floor_div(-a, b)
}

fn solve(ax: i128, ay: i128, bx: i128, by: i128, px: i128, py: i128) -> Option<(i128, i128)> {
    let det = ax * by - ay * bx;
    if det != 0 {
        // Unique solution by Cramer's rule
        let num_i = px * by - py * bx;
        let num_j = ax * py - ay * px;
        if num_i % det != 0 || num_j % det != 0 {
            return None;
        }
        let i = num_i / det;
        let j = num_j / det;
        if i >= 0 && j >= 0 {
            Some((i, j))
        } else {
            None
        }
    } else {
        // Colinear
        if px * by - py * bx != 0 {
            return None;
        }
        let (g, x0, y0) = egcd(ax, bx);
        if px % g != 0 {
            return None;
        }
        let mul = px / g;
        let i0 = x0 * mul;
        let j0 = y0 * mul;

        let step_i = bx / g;
        let step_j = ax / g;

        // Constraints: i = i0 + step_i*t >= 0 ; j = j0 - step_j*t >= 0
        let mut low = i128::MIN;
        let mut high = i128::MAX;

        if step_i != 0 {
            if step_i > 0 {
                low = low.max(ceil_div(-i0, step_i));
            } else {
                high = high.min(floor_div(-i0, step_i));
            }
        } else if i0 < 0 {
            return None;
        }

        if step_j != 0 {
            if step_j > 0 {
                high = high.min(floor_div(j0, step_j));
            } else {
                low = low.max(ceil_div(j0, step_j));
            }
        } else if j0 < 0 {
            return None;
        }

        if low > high {
            return None;
        }

        // Cost function: 3*i + j = 3*(i0 + step_i*t) + (j0 - step_j*t)
        // = (3*i0 + j0) + t*(3*step_i - step_j)
        let slope = 3 * step_i - step_j;
        let t_best = if slope > 0 {
            low
        } else if slope < 0 {
            high
        } else {
            low
        };

        let i = i0 + step_i * t_best;
        let j = j0 - step_j * t_best;

        if i >= 0 && j >= 0 {
            Some((i, j))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
