advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut current = 50;
    let mut score = 0;

    for line in input.lines() {
        let (dir, step) = parse_line(line)?;
        match dir {
            Direction::Left => current = (current - step) % 100,
            Direction::Right => current = (current + step) % 100,
        }

        if current == 0 {
            score += 1;
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current = 50;
    let mut score: u64 = 0;

    for line in input.lines() {
        let (dir, step) = parse_line(line)?;

        score += (step / 100) as u64;

        let step = step.rem_euclid(100);

        if current != 0 {
            let dist = match dir {
                Direction::Left => current,
                Direction::Right => 100 - current,
            };
            if step >= dist {
                score += 1;
            }
        }

        current = match dir {
            Direction::Left => (current - step).rem_euclid(100),
            Direction::Right => (current + step).rem_euclid(100),
        };
    }

    Some(score)
}

fn parse_line(line: &str) -> Option<(Direction, i64)> {
    let mut chars = line.chars();
    let dir = match chars.next()? {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => return None,
    };
    let num = chars.as_str().parse().ok()?;
    Some((dir, num))
}

enum Direction {
    Left,
    Right,
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
        assert_eq!(result, Some(6));
    }
}
