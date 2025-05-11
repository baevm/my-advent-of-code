use fancy_regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines();

    let mut total_chars = 0;
    let mut total_chars_in_mem = 0;

    let filter_regex = Regex::new(r"\\x[a-f0-9]{2}").unwrap();

    for line in lines {
        total_chars += line.len();

        let filtered_line = line
            .trim_matches('"')
            .replace("\\\"", "A")
            .replace("\\\\", "B");

        let filtered_regex_line = filter_regex.replace_all(&filtered_line, "C");

        total_chars_in_mem += filtered_regex_line.len();
    }

    Some(total_chars - total_chars_in_mem)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines();

    let result = lines.fold(0, |acc, line| {
        let filtered_len = line.replace("\\", "AA").replace("\"", "BB").len() + 2; // 2 - кавычки (1 слева + 1 справа)
        acc + filtered_len - line.len()
    });

    Some(result)
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
