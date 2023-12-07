use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn get_type_from_match_counts(match_counts: &Vec<u32>) -> u32 {
    // 5 of a kind
    if match_counts.contains(&5) {
        return 6;
    }

    // 4 of a kind
    if match_counts.contains(&4) {
        return 5;
    }

    if match_counts.contains(&3) {
        if match_counts.contains(&2) {
            // full house
            return 4;
        }
        return 3;
    }

    if match_counts.contains(&2) {
        if match_counts.iter().filter(|c| c == &&2).count() > 1 {
            // two pair
            return  2;
        }
        // one pair
        return 1;
    }

    0
}

pub fn get_hand_type(hand: [u32; 5], j_wild: bool) -> (u32, [u32; 5]) {
    let mut cards: HashMap<u32, u32> = HashMap::new();
    let mut max_key = 2;
    let mut max_val = 0;
    let mut next_hand = hand.clone();

    for card in hand {
        let count = &cards.get(&card).unwrap_or(&0);
        let next_count = *count + 1;

        if next_count > max_val && (!j_wild || (j_wild && card != 11)) {
            max_val = next_count;
            max_key = card;
        }

        cards.insert(card, next_count);
    }

    if j_wild {
        let wild_count = cards.get(&11).unwrap_or(&0).to_owned();
        cards.remove_entry(&11);
        cards.insert(max_key, max_val + wild_count);

        for (i, card) in hand.into_iter().enumerate() {
            if card == 11 {
                next_hand[i] = 1;
            }
        }
    }

    let match_counts: Vec<u32> = cards.clone().into_values().collect();
    let hand_type = get_type_from_match_counts(&match_counts);

    (hand_type, next_hand)
}

pub fn parse_input(input: &str, j_wild: bool) -> Vec<([u32; 5], u32, u32)> {
    let mut coll: Vec<([u32; 5], u32, u32)> = vec![];

    for line in input.lines() {
        let data: Vec<&str> = line.split(" ").collect();
        let mut data_iter = data.into_iter();
        let hand: Vec<u32> = data_iter
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                c.to_string()
                    .replace("T", "10")
                    .replace("J", "11")
                    .replace("Q", "12")
                    .replace("K", "13")
                    .replace("A", "14")
                    .parse()
                    .unwrap()
            })
            .collect();

        let hand: [u32; 5] = hand.try_into().unwrap();

        let bet: u32 = data_iter.next().unwrap().parse().unwrap();
        let (hand_type, next_hand) = get_hand_type(hand, j_wild);

        coll.push((next_hand, bet, hand_type))
    }

    coll
}

pub fn sort_hands(hands: &mut Vec<([u32; 5], u32, u32)>) -> () {
    hands.sort_by(|a, b| {
        let (hand_a, _bet_a, type_a) = a;
        let (hand_b, _bet_b, type_b) = b;

        if type_a != type_b {
            return type_a.cmp(type_b);
        }

        for (idx, card_a) in hand_a.into_iter().enumerate() {
            let card_b = hand_b[idx];
            if card_a != &card_b {
                return card_a.cmp(&card_b);
            }
        }

        return std::cmp::Ordering::Equal;
    });
}

pub fn count_winnings(hands: Vec<([u32; 5], u32, u32)>) -> u32 {
    let mut total = 0;

    for (i, hand) in hands.into_iter().enumerate() {
        let (_hand, bet, _hand_type) = hand;
        let value = bet * (i + 1) as u32;
        total += value;
    }

    total
}

pub fn debug_hands(hands: Vec<([u32; 5], u32, u32)>) -> () {
    for hand in hands {
        let (hand, _, hand_type) = hand;
        let kind = match hand_type {
            0 => "high",
            1 => "one pair",
            2 => "two pair",
            3 => "three of a kind",
            4 => "full house",
            5 => "four of a kind",
            6 => "five of a kind",
            _ => "",
        };

        println!("hand {:?} kind {}", hand, kind);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = parse_input(input, false);
    sort_hands(&mut hands);
    // debug_hands(hands.clone());
    let total = count_winnings(hands);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = parse_input(input, true);
    sort_hands(&mut hands);
    // debug_hands(hands.clone());
    let total = count_winnings(hands);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
