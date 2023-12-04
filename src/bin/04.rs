use regex::Regex;
use std::collections::HashSet;

advent_of_code::solution!(4);

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"\d+").unwrap();
    let card_values: Vec<[u32; 36]> = input
        .lines()
        .map(|line| {
            let res: [u32; 36] = re.find_iter(line)
                .map(|s| s.as_str().parse().unwrap_or(0))
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();
            res
        })
        .collect();

    let winning_number_sets: Vec<[u32; 10]> = card_values
        .clone()
        .into_iter()
        .map(|card| {
            let res: [u32; 10] = card[1..11].try_into().unwrap();
            res
        })
        .collect();

    let player_number_sets: Vec<[u32; 25]> = card_values
        .clone()
        .into_iter()
        .map(|card| {
            let res: [u32; 25] = card[11..].try_into().unwrap();
            res
        })
        .collect();

    let winning_values: Vec<Vec<u32>> = player_number_sets
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, card)| {
            let winning: HashSet<u32> = winning_number_sets[idx].into_iter().collect();
            let player: HashSet<u32> = card.into_iter().collect();
            let matches = winning.intersection(&player);
            matches.cloned().collect()
        })
        .collect();

    let points: Vec<u32> = winning_values.into_iter().map(|card| card.iter().fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })).collect();

    Some(points.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
