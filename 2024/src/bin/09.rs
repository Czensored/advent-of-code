advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut sum = 0;

    let mut initial_state = get_initial_state(&input);

    let mut final_state = Vec::new();
    let mut i = 0;

    while i < initial_state.len() {
        match initial_state[i] {
            Some(num) => {
                final_state.push(num);
                i += 1;
            }
            None => {
                loop {
                    match initial_state.pop() {
                        Some(Some(num)) => {
                            final_state.push(num);
                            break;
                        }
                        Some(None) => continue,
                        None => break,
                    }
                }
                initial_state.remove(i);
            }
        }
    }

    for (ind, i) in final_state.iter().enumerate() {
        sum += ind * i;
    }

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let mut file_info = Vec::new();

    let mut start_ind = 0;
    let mut id = 0;
    for (ind, &val) in input.iter().enumerate() {
        if ind % 2 == 0 {
            file_info.push((id, start_ind as usize, val as usize));
            id += 1;
        }
        start_ind += val;
    }

    file_info.reverse();

    let mut blocks = get_initial_state(&input);

    for (id, start, len) in file_info {
        // Look left of the file for the first free span of size >= len
        let mut i = 0;
        while i + len <= start {
            if blocks[i..i + len].iter().all(|b| b.is_none()) {
                // Move file here
                for j in 0..len {
                    blocks[i + j] = Some(id);
                    blocks[start + j] = None;
                }
                break;
            }
            i += 1;
        }
    }

    let sum: usize = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|id| i * id))
        .sum();

    Some(sum as u64)
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect()
}

fn get_initial_state(input: &[u8]) -> Vec<Option<usize>> {
    input
        .iter()
        .enumerate()
        .flat_map(|(ind, &num)| {
            let value = if ind % 2 == 0 { Some(ind / 2) } else { None };
            std::iter::repeat(value).take(num as usize)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
