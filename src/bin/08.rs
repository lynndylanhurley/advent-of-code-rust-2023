use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
pub struct Node {
    left: String,
    right: String,
}

pub fn parse_input(input: &str) -> (Vec<char>, HashMap<String, Node>) {
    let mut lines = input.lines();
    let addr_re = Regex::new(r"\w+").unwrap();
    let directions = lines.next().unwrap().chars().collect();
    let mut nodes = HashMap::new();

    // pass over empty space
    lines.next();

    let addresses: Vec<[String; 3]> = lines
        .map(|line| {
            let addr: Vec<String> = addr_re
                .find_iter(line)
                .map(|s| s.as_str().to_string())
                .collect();
            let addr: [String; 3] = addr.try_into().unwrap();
            addr
        })
        .collect();

    // initialize nodes
    for [addr, left, right] in &addresses {
        nodes.insert(
            addr.to_string(),
            Node {
                left: left.to_owned(),
                right: right.to_owned(),
            },
        );
    }

    (directions, nodes)
}

pub fn walk_and_count_steps(directions: Vec<char>, nodes: HashMap<String, Node>) -> u32 {
    let mut steps = 0;
    let mut cursor = "AAA";

    for dir in directions.iter().cycle() {
        if cursor == "ZZZ" {
            break;
        }

        steps += 1;
        let node = nodes.get(cursor).unwrap();
        cursor = if dir == &'L' { &node.left } else { &node.right };
    }

    steps
}

pub fn walk_multiple_paths_and_count_steps(
    directions: Vec<char>,
    nodes: HashMap<String, Node>,
) -> u128 {
    let mut steps = 0;
    let mut cursors: Vec<String> = nodes
        .keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .cloned()
        .collect();

    let mut cycle_lengths: Vec<u32> = vec![];

    for dir in directions.iter().cycle() {
        if cursors.is_empty() {
            break;
        }

        let terminal_cursors: Vec<String> = cursors
            .clone()
            .into_iter()
            .filter(|c| c.chars().last().unwrap() == 'Z')
            .collect();
        let terminal_count = terminal_cursors.len();

        for _ in 0..terminal_count {
            cycle_lengths.push(steps.clone());
        }

        steps += 1;

        cursors = cursors
            .into_iter()
            .filter(|c| c.chars().last().unwrap() != 'Z')
            .map(|c| {
                let node = nodes.get(&c).unwrap();
                if dir == &'L' { &node.left } else { &node.right }
            })
            .cloned()
            .collect();
    }

    let max_cycle = cycle_lengths.clone().iter().max().unwrap().clone();
    let mut count: u128 = max_cycle.into();

    'outer: loop {
        for cycle in cycle_lengths.clone() {
            if count % cycle as u128 != 0 {
                count += max_cycle as u128;
                continue 'outer;
            }
        }

        return count;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = parse_input(input);
    let steps = walk_and_count_steps(directions, nodes);
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (directions, nodes) = parse_input(input);
    let steps = walk_multiple_paths_and_count_steps(directions, nodes);
    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
