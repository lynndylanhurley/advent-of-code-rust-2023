use regex::Regex;

advent_of_code::solution!(2);

#[derive(Debug)]
pub struct GameResult {
    green: u32,
    red: u32,
    blue: u32,
}

pub fn results_by_color(input: &str, color: &str) -> Vec<u32> {
    let pattern = format!(r"(\d+) {}", color);
    let re = Regex::new(&pattern).unwrap();

    re.captures_iter(&input)
        .map(|cap| {
            cap.iter().map(|c| c.unwrap().as_str().parse().unwrap_or(0)).collect::<Vec<u32>>()
        })
        .flatten()
        .collect()
}


pub fn line_to_game_maxes(input: &str) -> GameResult {
    let reds = results_by_color(input, "red");
    let greens = results_by_color(input, "green");
    let blues = results_by_color(input, "blue");

    GameResult {
        green: *greens.iter().max().unwrap(),
        red: *reds.iter().max().unwrap(),
        blue: *blues.iter().max().unwrap(),
    }
}

pub fn line_to_game_min_pows(input: &str) -> u32 {
    let reds = results_by_color(input, "red");
    let greens = results_by_color(input, "green");
    let blues = results_by_color(input, "blue");

    let red_max = reds.iter().filter(|x| x != &&0).max().unwrap_or(&1);
    let green_max = greens.iter().filter(|x| x != &&0).max().unwrap_or(&1);
    let blue_max = blues.iter().filter(|x| x != &&0).max().unwrap_or(&1);

    let product = red_max * green_max * blue_max;

    product
}

pub fn is_possible(gm: &GameResult, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    if gm.red > max_red {
        return false;
    }

    if gm.blue > max_blue {
        return false;
    }

    if gm.green > max_green {
        return false;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let maxes_per_game: Vec<GameResult> = input.lines().map(|s| line_to_game_maxes(s)).collect();

    let mut sum = 0;

    for (index, gm) in maxes_per_game.iter().enumerate() {
        if is_possible(gm, 12, 13, 14) {
            sum += index + 1;
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let min_game_pows: Vec<u32> = input.lines().map(|s| line_to_game_min_pows(s)).collect();
    Some(min_game_pows.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
