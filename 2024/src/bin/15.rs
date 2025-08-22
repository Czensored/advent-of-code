advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut grid, movements) = parse_input(input);

    let mut robot = find_robot(&grid);

    for mov in movements.chars() {
        update_grid_part_one(&mut grid, &mut robot, mov);
    }

    let mut sum = 0;

    for (row, line) in grid.iter().enumerate() {
        for (col, &item) in line.iter().enumerate() {
            if item == 'O' {
                sum += 100 * row + col;
            }
        }
    }

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (input_grid, movements) = parse_input(input);

    let mut grid: Vec<Vec<char>> = input_grid
        .iter()
        .map(|line| {
            line.iter()
                .copied()
                .flat_map(|ch| match ch {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => unreachable!("Invalid input"),
                })
                .collect()
        })
        .collect();

    let mut robot = find_robot(&grid);

    for mov in movements.chars() {
        update_grid_part_two(&mut grid, &mut robot, mov);
    }

    let mut sum = 0;

    for (row, line) in grid.iter().enumerate() {
        for (col, &item) in line.iter().enumerate() {
            if item == 'O' {
                sum += 100 * row + col;
            }
        }
    }

    Some(sum as u64)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, String) {
    let mut lines = input.lines();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines.by_ref() {
        let l = line.trim_end();
        if l.is_empty() {
            break;
        }
        grid.push(l.chars().collect());
    }

    let mut moves = String::new();
    for line in lines {
        for c in line.chars() {
            if matches!(c, '<' | '>' | '^' | 'v') {
                moves.push(c);
            }
        }
    }

    (grid, moves)
}

fn find_robot(grid: &[Vec<char>]) -> (usize, usize) {
    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|&ch| ch == '@') {
            return (r, c);
        }
    }
    unreachable!("robot '@' not found");
}

fn update_grid_part_one(grid: &mut [Vec<char>], robot: &mut (usize, usize), ch: char) {
    let (dx, dy) = match ch {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => unreachable!("Non directional instruction found"),
    };

    let (rx, ry) = *robot;

    let nx = (rx as isize + dx) as usize;
    let ny = (ry as isize + dy) as usize;

    if grid[nx][ny] == '#' {
        return;
    } else if grid[nx][ny] == '.' {
        grid[nx][ny] = '@';
        *robot = (nx, ny);
        grid[rx][ry] = '.';
        return;
    } else {
        let mut counter = 1;
        loop {
            let nnx = (rx as isize + dx * counter) as usize;
            let nny = (ry as isize + dy * counter) as usize;
            if grid[nnx][nny] == '#' {
                return;
            } else if grid[nnx][nny] == '.' {
                grid[nnx][nny] = 'O';
                grid[nx][ny] = '@';
                *robot = (nx, ny);
                grid[rx][ry] = '.';
                return;
            }

            counter += 1;
        }
    }
}

fn update_grid_part_two(grid: &mut [Vec<char>], robot: &mut (usize, usize), ch: char) {
    let (dx, dy) = match ch {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => unreachable!("Non directional instruction found"),
    };

    let (rx, ry) = *robot;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
