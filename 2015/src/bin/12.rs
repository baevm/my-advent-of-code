use serde_json::Value;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    let jsoned: Value = serde_json::from_str(input).expect("failed to parse json");
    let mut res: i64 = 0;

    count_nums_part1(&jsoned, &mut res);

    Some(res)
}

fn count_nums_part1(input: &Value, res: &mut i64) {
    match input {
        Value::Array(map) => {
            for value in map.iter() {
                count_nums_part1(value, res);
            }
        }
        Value::Number(n) => {
            *res += n.as_i64().expect("failed to parse JSON number");
        }
        Value::Object(map) => {
            for (_, value) in map.iter() {
                count_nums_part1(value, res);
            }
        }
        _ => return,
    }
}

fn count_nums_part2(input: &Value, res: &mut i64) {
    match input {
        Value::Array(map) => {
            for value in map.iter() {
                count_nums_part2(value, res);
            }
        }
        Value::Number(n) => {
            *res += n.as_i64().expect("failed to parse JSON number");
        }
        Value::Object(map) => {
            for (_, value) in map.iter() {
                if value == "red" {
                    return;
                }
            }

            for (_, value) in map.iter() {
                count_nums_part2(value, res);
            }
        }
        _ => return,
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let jsoned: Value = serde_json::from_str(input).expect("failed to parse json");
    let mut res: i64 = 0;

    count_nums_part2(&jsoned, &mut res);

    Some(res)
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
