use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<String> {
    solve(input, 1)
}

pub fn part_two(input: &str) -> Option<String> {
    solve(input, 2)
}

fn solve(input: &str, retries: u64) -> Option<String> {
    let mut curr_password: Vec<u8> = input.as_bytes().to_vec();
    let n = input.len();

    let mut result: Option<Vec<u8>> = None;
    let mut curr_try = 0;

    while curr_try < retries {
        for i in (0..n).rev() {
            if curr_password[i] == b'z' {
                curr_password[i] = b'a';
            } else {
                curr_password[i] += 1;
                break;
            }
        }

        let curr_password_str: Vec<String> = curr_password
            .iter()
            .map(|&b| (b as char).to_string())
            .collect();

        let is_first_ok = check_first_req(&curr_password_str);
        let is_second_ok = check_second_req(&curr_password_str);
        let is_third_ok = check_third_req(&curr_password_str);

        if is_first_ok && is_second_ok && is_third_ok {
            result = Some(curr_password.clone());
            curr_try += 1;
        }
    }

    result.map(|bytes| String::from_utf8(bytes).unwrap())
}

// must contain atleast 2 different pairs of letters: 'aa', 'bb', 'cc', etc.
fn check_third_req(curr_password: &Vec<String>) -> bool {
    let mut pairs: HashSet<String> = HashSet::new();

    for (curr, next) in curr_password.iter().zip(curr_password.iter().skip(1)) {
        if curr == next {
            let pair = curr.clone() + next;

            pairs.insert(pair);

            if pairs.len() == 2 {
                return true;
            }
        }
    }

    false
}

// may not contain letters 'i' 'o' 'l'
fn check_second_req(curr_password: &Vec<String>) -> bool {
    return !curr_password
        .iter()
        .any(|letter| letter == "i" || letter == "o" || letter == "l");
}

// must contain increasing 3 letters: 'abc'
fn check_first_req(curr_password: &Vec<String>) -> bool {
    for i in 0..curr_password.len() - 3 {
        let curr = curr_password[i].as_bytes().first().unwrap();
        let next = curr_password[i + 1].as_bytes().first().unwrap();
        let next_next = curr_password[i + 2].as_bytes().first().unwrap();

        if curr + 1 == *next && next + 1 == *next_next {
            return true;
        }
    }

    false
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
