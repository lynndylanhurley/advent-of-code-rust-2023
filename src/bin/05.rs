use regex::Regex;

advent_of_code::solution!(5);

pub fn listing_src_to_dest(listing: &Vec<[u32; 3]>, source: u32) -> u32 {
    for entry in listing.iter() {
        if source >= entry[1] && source < entry[1] + entry[2] {
            let offset = source - entry[1];
            let dest = entry[0] + offset;
            return dest;
        }
    }

    source
}

pub fn seed_to_location(listings: &Vec<Vec<[u32; 3]>>, source: u32) -> u32 {
    let mut cursor = source;

    for listing in listings.iter() {
        cursor = listing_src_to_dest(listing, cursor);
    }

    cursor
}

pub fn get_listings_from_input(input: &str) -> Vec<Vec<[u32; 3]>> {
    let newline_re = Regex::new(r"\n\n").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();
    let listings: Vec<&str> = newline_re.split(input).collect();
    let mut results = vec![];

    for (i, listing) in listings.iter().enumerate() {
        // ignore seeds list
        if i == 0 {
            continue;
        }

        let mut lines = listing.lines();

        // skip name
        lines.next();

        let mut coll: Vec<[u32; 3]> = vec![];

        for line in lines {
            let numbers: Vec<u32> = number_re
                .find_iter(line).map(|s| s.as_str().parse().unwrap())
                .collect();

            let map: [u32; 3] = numbers.try_into().unwrap();

            coll.push(map);
        }

        results.push(coll);
    }

    results
}

pub fn get_seeds_from_input(input: &str) -> Vec<u32> {
    let seeds_re = Regex::new(r"\d+").unwrap();
    let first_line = input.lines().next().unwrap();
    seeds_re.find_iter(first_line).map(|s| s.as_str().parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let seeds = get_seeds_from_input(input);
    let listings = get_listings_from_input(input);

    let locations: Vec<u32> = seeds.iter().map(|seed| seed_to_location(&listings, *seed)).collect();

    Some(locations.into_iter().min().unwrap())
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
