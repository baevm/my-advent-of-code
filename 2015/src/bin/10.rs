advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 40)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 50)
}

fn solve(input: &str, rounds: i64) -> Option<usize> {
    let mut sb = String::from(input);

    for _ in 0..rounds {
        let mut new_sb = String::new();

        let mut curr_ch = sb.chars().nth(0).unwrap();
        let mut curr_ch_count = 1;

        for curr_in_sb in sb.chars().skip(1) {
            if curr_in_sb != curr_ch {
                new_sb.push_str(&format!("{curr_ch_count}{curr_ch}"));
                curr_ch = curr_in_sb;
                curr_ch_count = 1;
            } else {
                curr_ch_count += 1;
            }
        }

        // оставшееся число в конце
        new_sb.push_str(&format!("{curr_ch_count}{curr_ch}"));

        sb = new_sb;
    }

    Some(sb.len())
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
