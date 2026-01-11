advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    for mut line in input {
        let mut max = 0;
        for i in 1..line.len() {
            if line[max] < line[i] {
                max = i
            }
        }

        if max != line.len() - 1 {
            sum += (line[max] * 10) as u64;
            let new_line = &line[max + 1..];
            sum += *new_line.iter().max().unwrap() as u64;
        } else {
            sum += line[max] as u64;
            line.remove(max);
            sum += (*line.iter().max().unwrap() * 10) as u64;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    for line in input {
        let mut num_cells = 12;
        let mut max_ind = 0;

        while num_cells > 1 {
            for i in 1 + max_ind..line.len() - num_cells + 1 {
                if line[max_ind] < line[i] {
                    max_ind = i
                }
            }

            sum += line[max_ind] as u64 * 10u64.pow(num_cells as u32 - 1);
            num_cells -= 1;
            max_ind += 1;
        }

        sum += *line[max_ind..].iter().max().unwrap() as u64
    }

    Some(sum)
}
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
