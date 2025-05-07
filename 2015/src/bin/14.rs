use std::collections::HashMap;

use fancy_regex::Regex;
use itertools::Itertools;

advent_of_code::solution!(14);

struct Reindeer {
    speed: i64,
    burst_time: i64,
    rest_time: i64,
}

#[derive(Debug)]
struct ReindeerRacer {
    distance: i64,
    points: i64,
    burst_time_left: i64,
    rest_time_left: i64,
}

pub fn part_one(input: &str) -> Option<i64> {
    return calculate_result(input, false);
}

pub fn part_two(input: &str) -> Option<i64> {
    return calculate_result(input, true);
}

fn parse_regex_as_i64(regex: &Regex, line: &str) -> i64 {
    return regex
        .captures(line)
        .expect("Error running regex")
        .expect("reindeer name not found")
        .get(0)
        .expect("no group")
        .as_str()
        .parse()
        .expect("failed to parse speed");
}

fn calculate_result(input: &str, is_part2: bool) -> Option<i64> {
    let lines = input.lines();

    // Example: "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."

    // returns 16
    let speed_regex = Regex::new(r"[0-9]*\w(?=\skm\/s)").unwrap();
    // returns 11
    let burst_time_regex = Regex::new(r"(?<=for\s)[0-9]*(?=\sseconds,)").unwrap();
    // returns 162
    let rest_time_regex = Regex::new(r"(?<=for\s)[0-9]*(?=\sseconds\.)").unwrap();
    // returns Dancer
    let reindeer_name_regex = Regex::new(r"\w+(?=\scan)").unwrap();

    let mut reindeers: HashMap<&str, Reindeer> = HashMap::new();

    for line in lines {
        let reindeer_name = reindeer_name_regex
            .captures(line)
            .expect("Error running regex")
            .expect("reindeer name not found")
            .get(0)
            .expect("no group")
            .as_str();

        let speed: i64 = parse_regex_as_i64(&speed_regex, line);
        let burst_time: i64 = parse_regex_as_i64(&burst_time_regex, line);
        let rest_time: i64 = parse_regex_as_i64(&rest_time_regex, line);

        reindeers.insert(
            reindeer_name,
            Reindeer {
                speed,
                burst_time,
                rest_time,
            },
        );
    }

    // amount of seconds
    const TIME: i64 = 2503;
    let mut reindeer_distances: HashMap<&str, ReindeerRacer> = HashMap::new();

    for _ in 0..TIME {
        for (reindeer_name, stats) in reindeers.iter() {
            if !reindeer_distances.contains_key(reindeer_name) {
                reindeer_distances.insert(
                    &reindeer_name,
                    ReindeerRacer {
                        distance: 0,
                        burst_time_left: stats.burst_time,
                        rest_time_left: 0,
                        points: 0,
                    },
                );
            }

            let reinder_racer = reindeer_distances.get_mut(reindeer_name).unwrap();

            // Начало отдыха - оба счетчика по 0
            if reinder_racer.burst_time_left == 0 && reinder_racer.rest_time_left == 0 {
                reinder_racer.rest_time_left = stats.rest_time;
            } else if reinder_racer.burst_time_left == 0 {
                // Отдых закончился
                reinder_racer.burst_time_left = stats.burst_time;
            }

            // Отдых идет
            if reinder_racer.rest_time_left > 0 {
                reinder_racer.rest_time_left -= 1;
            } else if reinder_racer.burst_time_left > 0 {
                // Полет идет
                reinder_racer.burst_time_left -= 1;
                reinder_racer.distance += stats.speed;
            }
        }

        let max_dist = reindeer_distances
            .iter()
            .max_set_by(|a, b| a.1.distance.cmp(&b.1.distance));

        let reindeer_names_to_award: Vec<String> = max_dist
            .iter()
            .map(|reindeer| reindeer.0.to_string())
            .collect();

        for reindeer_name in reindeer_names_to_award {
            reindeer_distances
                .get_mut(reindeer_name.as_str())
                .map(|racer| racer.points += 1);
        }
    }

    if !is_part2 {
        let mut max_distance = 0;

        for (_, stats) in reindeer_distances.iter() {
            max_distance = max_distance.max(stats.distance);
        }

        Some(max_distance)
    } else {
        let mut max_points = 0;

        for (_, stats) in reindeer_distances.iter() {
            max_points = max_points.max(stats.points);
        }

        Some(max_points)
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
