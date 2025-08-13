use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

struct Node {
    weight: i64,
    children: Vec<String>,
}

fn solve(input: &str, is_part_2: bool) -> Option<String> {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    let mut all_nodes: HashSet<String> = HashSet::new();
    let mut child_nodes: HashSet<String> = HashSet::new();

    for line in input.lines() {
        let parts: Vec<String> = line
            .split_whitespace()
            .map(|item| item.to_string())
            .collect();

        let weight = parts[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .parse::<i64>()
            .unwrap();

        // if length > 2 -> contains childs
        let children = if parts.len() > 2 {
            parts
                .iter()
                .skip(3)
                .map(|child| child.trim_end_matches(',').to_string())
                .collect()
        } else {
            vec![]
        };

        nodes.insert(
            parts[0].clone(),
            Node {
                children: children.clone(),
                weight,
            },
        );

        all_nodes.insert(parts[0].clone());

        children.into_iter().for_each(|child| {
            child_nodes.insert(child);
        });
    }

    let mut root: Option<String> = None;

    for name in all_nodes {
        if !child_nodes.contains(&name) {
            root = Some(name);
        }
    }

    if !is_part_2 {
        return root;
    }

    walk_nodes(&root.unwrap(), &nodes);

    None
}

fn get_weight(node_name: &str, nodes: &HashMap<String, Node>) -> i64 {
    let curr_node = nodes.get(node_name).expect("failed to find node");
    let mut total = curr_node.weight;

    for child in &curr_node.children {
        total += get_weight(&child, nodes)
    }

    total
}

fn walk_nodes(root_name: &str, nodes: &HashMap<String, Node>) {
    let node = nodes.get(root_name).expect("failed to find node");

    let weights: Vec<i64> = node
        .children
        .iter()
        .map(|child| get_weight(child, &nodes))
        .collect();

    for weight in &weights {
        if *weight != weights[0] {
            println!("invalid weight: {} !== {}", weight, weights[0]);

            println!(
                "{} ({}) -> {:?}",
                root_name,
                node.weight,
                node.children.iter().zip(weights).collect::<Vec<_>>()
            );

            break;
        }
    }

    for node_child in &node.children {
        walk_nodes(node_child, nodes);
    }
}

pub fn part_one(input: &str) -> Option<String> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<String> {
    solve(input, true)
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
