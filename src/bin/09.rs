advent_of_code::solution!(9);

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|s| s
            .split(" ")
            .map(|c| c.parse().unwrap())
            .collect()
        ).collect()
}

pub fn differences(history: &Vec<i32>) -> Vec<i32> {
    let mut diffs: Vec<i32> = vec![];
    let mut last = 0;

    for (i, record) in history.iter().enumerate() {
        if i != 0 {
            diffs.push(record - last);
        }

        last = *record;
    }

    diffs
}

pub fn extrapolate_history(history: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut sequences: Vec<Vec<i32>> = vec![history.to_vec()];
    let mut last: Vec<i32> = history.to_vec();

    loop {
        let diffs = differences(&last);
        sequences.push(diffs.clone());

        let max = diffs.iter().max().unwrap();
        let min = diffs.iter().min().unwrap();

        if max == &0 && min == &0 {
            break;
        }

        last = diffs;
    }

    sequences
}

pub fn predict_next(sequence_data: &Vec<Vec<i32>>) -> i32 {
    let mut cursor = 0;

    for seq in sequence_data.iter().rev() {
        let last = seq.last().unwrap();
        let next = cursor + last;
        cursor = next;
    }

    cursor
}

pub fn predict_first(sequence_data: &Vec<Vec<i32>>) -> i32 {
    let mut cursor = 0;

    for seq in sequence_data.iter().rev() {
        let last = seq.first().unwrap();
        let next = last - cursor;
        cursor = next;
    }

    cursor
}

pub fn part_one(input: &str) -> Option<i32> {
    let histories = parse_input(input);
    let data: Vec<Vec<Vec<i32>>> = histories.iter().map(|h| extrapolate_history(h)).collect();
    let predictions: Vec<i32> = data.iter().map(|seq_data| predict_next(seq_data)).collect();
    let total = predictions.iter().sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let histories = parse_input(input);
    let data: Vec<Vec<Vec<i32>>> = histories.iter().map(|h| extrapolate_history(h)).collect();
    let predictions: Vec<i32> = data.iter().map(|seq_data| predict_first(seq_data)).collect();
    let total = predictions.iter().sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
