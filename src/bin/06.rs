use regex::Regex;

advent_of_code::solution!(6);

pub fn get_numbers_from_line(line: &str) -> Vec<u128> {
    let number_re = Regex::new(r"\d+").unwrap();
    number_re
        .find_iter(line)
        .map(|n| n.as_str().parse().unwrap())
        .collect()
}

pub fn parse_input_to_records(input: &str) -> Vec<(u128, u128)> {
    let mut lines = input.lines();
    let times = get_numbers_from_line(lines.next().unwrap());
    let distances = get_numbers_from_line(lines.next().unwrap());
    times.into_iter().zip(distances.into_iter()).collect()
}

pub fn get_winning_strategy_count(record: (u128, u128)) -> u128 {
    let (time, winning_distance) = record;
    let mut counter = 0;

    for charge_time in 1..time {
        let cruising_time = time - charge_time;
        let cruising_distance = charge_time * cruising_time;

        if cruising_distance > winning_distance {
            counter += 1;
        }
    }

    counter
}

pub fn part_one(input: &str) -> Option<u128> {
    let records = parse_input_to_records(input);
    let winning_strat_counts: Vec<u128> = records
        .iter()
        .map(|r| get_winning_strategy_count(*r))
        .collect();

    let result = winning_strat_counts.iter().fold(1, |agg, x| agg * x);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u128> {
    let records = parse_input_to_records(&input.replace(" ", ""));
    let winning_strat_counts: Vec<u128> = records
        .iter()
        .map(|r| get_winning_strategy_count(*r))
        .collect();

    let result = winning_strat_counts.iter().fold(1, |agg, x| agg * x);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
