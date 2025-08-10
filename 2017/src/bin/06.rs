use std::collections::HashMap;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<i64> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve(input, true)
}

pub fn solve(input: &str, is_part_2: bool) -> Option<i64> {
    let mut banks: Vec<i64> = input
        .split_whitespace()
        .filter_map(|bank| bank.parse().ok())
        .collect();

    let mut cycles = 0;

    let mut used: HashMap<Vec<i64>, i64> = HashMap::new();
    let n = banks.len();

    loop {
        let (biggest_idx, &biggest) = banks
            .iter()
            .enumerate()
            .max_by(|(idx_a, val_a), (idx_b, val_b)| {
                val_a.cmp(val_b).then_with(|| idx_b.cmp(idx_a))
            })
            .unwrap();

        banks[biggest_idx] = 0;

        for i in 1..=biggest {
            let curr_idx = (biggest_idx + i as usize) % n;
            banks[curr_idx] += 1;
        }

        cycles += 1;

        if used.contains_key(&banks) {
            if is_part_2 {
                return Some(cycles - *used.get(&banks).unwrap());
            } else {
                break;
            }
        }

        used.insert(banks.clone(), cycles);
    }

    Some(cycles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
