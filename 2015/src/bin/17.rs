advent_of_code::solution!(17);

const TARGET_LITERS: i64 = 150;

pub fn part_one(input: &str) -> Option<i64> {
    let buckets: Vec<i64> = input
        .lines()
        .map(|b| b.parse::<i64>().expect("failed to parse bucket size"))
        .collect();

    let mut found_combos: Vec<Vec<i64>> = vec![];
    let mut current_combos: Vec<i64> = vec![];

    combinations(
        TARGET_LITERS,
        0,
        &mut current_combos,
        0,
        &buckets,
        &mut found_combos,
    );

    Some(found_combos.len() as i64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let buckets: Vec<i64> = input.lines().map(|b| b.parse::<i64>().unwrap()).collect();

    let mut found_combos: Vec<Vec<i64>> = vec![];
    let mut current_combos: Vec<i64> = vec![];

    combinations(
        TARGET_LITERS,
        0,
        &mut current_combos,
        0,
        &buckets,
        &mut found_combos,
    );

    found_combos.sort_by_key(|item| item.len());

    let minimum_buckets = found_combos.first().unwrap().len();
    let result = found_combos
        .iter()
        .take_while(|item| item.len() == minimum_buckets)
        .count();

    Some(result as i64)
}

fn combinations(
    target_liters: i64,
    curr_liters: i64,
    curr_buckets: &mut Vec<i64>,
    idx: usize,
    all_buckets: &Vec<i64>,
    result: &mut Vec<Vec<i64>>,
) {
    if curr_liters == target_liters {
        result.push(curr_buckets.clone());
        return;
    }

    if curr_liters > target_liters {
        return;
    }

    for new_idx in idx..all_buckets.len() {
        let bucket = all_buckets[new_idx];

        curr_buckets.push(bucket);

        combinations(
            target_liters,
            curr_liters + bucket,
            curr_buckets,
            new_idx + 1,
            all_buckets,
            result,
        );

        curr_buckets.pop();
    }
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
