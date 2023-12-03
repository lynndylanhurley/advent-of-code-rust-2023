use regex::Regex;

advent_of_code::solution!(3);

// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..

#[derive(Debug, Clone)]
pub struct PartNumber {
    digit: u32,
    left: [u32; 2],
    right: [u32; 2],
}

pub fn get_symbol_coordinates(input: &str, pattern: &str) -> Vec<[u32; 2]> {
    let re = Regex::new(pattern).unwrap();
    let mut coll: Vec<[u32; 2]> = [].to_vec();
    input.lines().enumerate().for_each(|(idx, line)| {
        re.find_iter(line)
            .for_each(|cap| coll.push([idx as u32, cap.start() as u32]))
    });

    coll
}

pub fn is_within_threshold(start: [u32; 2], end: [u32; 2], from: [u32; 2], threshold: u32) -> bool {
    let left = start[1] as i32;
    let right = end[1] as i32;
    let y = start[0] as i32;

    let point_x = from[1] as i32;
    let point_y = from[0] as i32;

    let dx = *[left - point_x, 0, point_x - right + 1]
        .to_vec()
        .iter()
        .max()
        .unwrap_or(&0) as u32;

    if dx > threshold {
        return false;
    }

    let dy = *[y - point_y, 0, point_y - y]
        .to_vec()
        .iter()
        .max()
        .unwrap_or(&0) as u32;

    if dy > threshold {
        return false;
    }

    return true;
}

pub fn get_digit_coordinates(input: &str) -> Vec<PartNumber> {
    let re = Regex::new(r"\d+").unwrap();
    let mut coll: Vec<PartNumber> = [].to_vec();

    input.lines().enumerate().for_each(|(idx, line)| {
        re.find_iter(line).for_each(|cap| {
            coll.push(PartNumber {
                digit: cap.as_str().parse().unwrap_or(0),
                left: [idx as u32, cap.start() as u32],
                right: [idx as u32, cap.end() as u32],
            })
        })
    });

    coll
}

pub fn filter_digits_by_symbol_distance(
    symbol_coords: &Vec<[u32; 2]>,
    digit_coords: &Vec<PartNumber>,
    max_distance: u32,
) -> Vec<u32> {
    let valid_digits: Vec<&PartNumber> = digit_coords
        .iter()
        .filter(|pn| {
            for s in symbol_coords {
                if is_within_threshold(pn.left, pn.right, *s, max_distance) {
                    return true;
                }
            }
            false
        })
        .collect();

    valid_digits.iter().map(|pn| pn.digit).collect()
}

pub fn filter_digits_adjacent_to_asterisk(
    symbol_coords: &Vec<[u32; 2]>,
    digit_coords: &Vec<PartNumber>,
    max_distance: u32,
) -> Vec<u32> {
    let mut coll: Vec<u32> = [].to_vec();
    for s in symbol_coords {
        let mut count = 0;
        let mut agg = 1;

        for pn in digit_coords {
            if is_within_threshold(pn.left, pn.right, *s, max_distance) {
                count += 1;
                agg *= pn.digit;

                if count > 2 {
                    break;
                }
            }
        }

        if count == 2 {
            coll.push(agg);
        }
    }

    coll
}

pub fn part_one(input: &str) -> Option<u32> {
    let symbol_coordinates = get_symbol_coordinates(input, r"[^\w\d\.]");
    let digit_coordinates = get_digit_coordinates(input);
    let filtered_digits =
        filter_digits_by_symbol_distance(&symbol_coordinates, &digit_coordinates, 1);

    let sum = filtered_digits.iter().sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let symbol_coordinates = get_symbol_coordinates(input, r"\*");
    let digit_coordinates = get_digit_coordinates(input);
    let filtered_digits = filter_digits_adjacent_to_asterisk(&symbol_coordinates, &digit_coordinates, 1);
    let sum = filtered_digits.iter().sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
