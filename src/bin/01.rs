use regex::{Captures, Regex};

advent_of_code::solution!(1);

pub fn doc_to_n(input: String) -> u32 {
    let mut tens = Some(0);
    let mut ones = Some(0);

    for c in input.chars() {
        if c.is_numeric() {
            tens = c.to_digit(10);
            break;
        }
    }

    for c in input.chars().rev() {
        if c.is_numeric() {
            ones = c.to_digit(10);
            break;
        }
    }

    tens.unwrap_or(0) * 10 + ones.unwrap_or(0)
}

pub fn doc_to_n_2(input: String) -> u32 {
    let mut tens = Some(0);
    let mut ones = Some(0);

    for c in parse_doc_fwd(input.clone()).chars() {
        if c.is_numeric() {
            tens = c.to_digit(10);
            break;
        }
    }

    for c in parse_doc_rev(input).chars() {
        if c.is_numeric() {
            ones = c.to_digit(10);
            break;
        }
    }

    tens.unwrap_or(0) * 10 + ones.unwrap_or(0)
}


pub fn parse_doc_fwd(input: String) -> String {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    re.replace(&input, |caps: &Captures| match &caps[0] {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "",
    }).to_string()
}

pub fn parse_doc_rev(input: String) -> String {
    let re = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    re.replace(&input.chars().rev().collect::<String>(), |caps: &Captures| match &caps[0] {
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        _ => "",
    }).to_string()
}

pub fn part_one(input: &str) -> Option<u32> {
    let docs: Vec<String> = input
        .lines()
        .map(|s| s.to_string())
        .collect();
    let digits: Vec<u32> = docs
        .into_iter()
        .map(|s| doc_to_n(s))
        .collect();

    Some(digits.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let docs: Vec<String> = input
        .lines()
        .map(|s| s.to_string())
        .collect();
    let digits: Vec<u32> = docs
        .into_iter()
        .map(|s| doc_to_n_2(s))
        .collect();

    Some(digits.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
