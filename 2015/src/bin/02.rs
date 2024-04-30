use std::cmp::min;

// https://adventofcode.com/2015/day/2
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines();
    let mut total_area = 0;

    for line in lines {
        let dimensions: Vec<i32> = line.split("x").map(|x| x.parse().unwrap()).collect();
        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
        let area = get_surface_area(l, w, h);
        total_area += area;
    }

    Some(total_area)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();
    let mut total_ribbon = 0;

    for line in lines {
        let mut dimensions: Vec<i32> = line.split("x").map(|x| x.parse().unwrap()).collect();
        dimensions.sort_unstable();
        let (side1, side2, side3) = (dimensions[0], dimensions[1], dimensions[2]);

        let ribbon_length = (side1 + side1 + side2 + side2) + (side1 * side2 * side3);
        total_ribbon += ribbon_length;
    }

    Some(total_ribbon)
}

fn get_surface_area(l: i32, w: i32, h: i32) -> i32 {
    let lw = l * w;
    let wh = w * h;
    let hl = h * l;
    let area = 2 * lw + 2 * wh + 2 * hl;
    let smallest_side = min(min(lw, wh), hl);

    return area + smallest_side;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
