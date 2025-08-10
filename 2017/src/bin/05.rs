advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, true))
}

pub fn solve(input: &str, is_part_2: bool) -> i64 {
    let mut steps = 0;

    let mut offsets: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().expect("failed to parse str to i64"))
        .collect();

    let mut curr_instr_idx: i64 = 0;

    while (curr_instr_idx as usize) < offsets.len() {
        let curr_instr = offsets[curr_instr_idx as usize];

        if is_part_2 && curr_instr >= 3 {
            offsets[curr_instr_idx as usize] -= 1;
        } else {
            offsets[curr_instr_idx as usize] += 1;
        }

        curr_instr_idx += curr_instr;
        steps += 1;
    }

    steps
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
