advent_of_code::solution!(15);

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

pub fn part_one(input: &str) -> Option<i64> {
    return Some(solve(input, false));
}

pub fn part_two(input: &str) -> Option<i64> {
    return Some(solve(input, true));
}

fn solve(input: &str, is_part_2: bool) -> i64 {
    let lines = input.lines();

    let mut ingredients: Vec<Ingredient> = vec![];

    for line in lines {
        // parses "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
        // to iter [2,3,-2,-1,3]
        let mut params = line
            .split_whitespace()
            .skip(1)
            .map(|param| param.trim_end_matches(|c| c == ',').to_string())
            .enumerate()
            .filter_map(|(i, x)| {
                if i % 2 != 0 {
                    Some(x.parse::<i64>().unwrap())
                } else {
                    None
                }
            });

        let capacity = params.next().unwrap();
        let durability = params.next().unwrap();
        let flavor = params.next().unwrap();
        let texture = params.next().unwrap();
        let calories = params.next().unwrap();

        ingredients.push(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        });
    }

    let mut max_points = 0;
    let mut max_points_with_500cal = 0;

    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                for h in 0..=100 - i - j - k {
                    let a = ingredients.get(0).unwrap();
                    let b = ingredients.get(1).unwrap();
                    let c = ingredients.get(2).unwrap();
                    let d = ingredients.get(3).unwrap();

                    let total_cap =
                        a.capacity * i + b.capacity * j + c.capacity * k + d.capacity * h;

                    let total_dur =
                        a.durability * i + b.durability * j + c.durability * k + d.durability * h;

                    let total_flav = a.flavor * i + b.flavor * j + c.flavor * k + d.flavor * h;

                    let total_tex = a.texture * i + b.texture * j + c.texture * k + d.texture * h;

                    let total_cal =
                        a.calories * i + b.calories * j + c.calories * k + d.calories * h;

                    if total_cap <= 0 || total_dur <= 0 || total_flav <= 0 || total_tex <= 0 {
                        continue;
                    }

                    let curr_points = total_cap * total_dur * total_flav * total_tex;

                    if total_cal == 500 {
                        max_points_with_500cal = max_points_with_500cal.max(curr_points);
                    }

                    max_points = max_points.max(curr_points);
                }
            }
        }
    }

    if !is_part_2 {
        max_points
    } else {
        max_points_with_500cal
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
