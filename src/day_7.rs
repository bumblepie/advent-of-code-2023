use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;
use std::ops::AddAssign;
use std::str::FromStr;

fn cmp_hands(
    hand: &str,
    other: &str,
    evaluate_hand_fn: fn(&str) -> HandType,
    card_value_fn: fn(&char) -> u64,
) -> Ordering {
    let hand_type_ordering = evaluate_hand_fn(hand).cmp(&evaluate_hand_fn(&other));
    if hand_type_ordering != Ordering::Equal {
        return hand_type_ordering;
    }

    for (self_card, other_card) in hand.chars().zip(other.chars()) {
        let self_card_value = card_value_fn(&self_card);
        let other_card_value = card_value_fn(&other_card);
        let card_ordering = self_card_value.cmp(&other_card_value);
        if card_ordering != Ordering::Equal {
            return card_ordering;
        }
    }

    return Ordering::Equal;
}

fn value_of_card_part_one(card: &char) -> u64 {
    match card {
        'A' => 20,
        'K' => 19,
        'Q' => 18,
        'J' => 17,
        'T' => 16,
        c => u64::from_str(c.to_string().as_str()).expect("Unexpected card"),
    }
}

fn value_of_card_part_two(card: &char) -> u64 {
    match card {
        'A' => 20,
        'K' => 19,
        'Q' => 18,
        'T' => 16,
        'J' => 0,
        c => u64::from_str(c.to_string().as_str()).expect("Unexpected card"),
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn evaluate_hand_part_one(hand: &str) -> HandType {
    let counts = hand.chars().counts();
    let max_count = counts.values().max().unwrap();
    match max_count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if counts.values().contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if counts.values().filter(|num| **num == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::Pair
            }
        }
        1 => HandType::HighCard,
        _ => panic!("Unexpected hand counts"),
    }
}

fn evaluate_hand_part_two(hand: &str) -> HandType {
    // Count non-jokers
    let mut counts = hand.chars().filter(|c| *c != 'J').counts();
    let joker_counts = hand.chars().filter(|c| *c == 'J').count();

    // Set jokers to match most common card (J if 5J hand)
    let max_key = counts
        .iter()
        .max_by_key(|(_card, count)| **count)
        .map(|(card, _count)| card.clone())
        .unwrap_or('J');
    counts.entry(max_key).or_insert(0).add_assign(joker_counts);
    let max_count = counts[&max_key];

    match max_count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if counts.values().contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if counts.values().filter(|num| **num == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::Pair
            }
        }
        1 => HandType::HighCard,
        _ => panic!("Unexpected hand counts"),
    }
}

fn parse_line(line: &str) -> (String, u64) {
    let sections = line.split_whitespace().collect_vec();
    let hand = sections[0].to_string();
    let bid = u64::from_str(sections[1]).expect("Unable to parse bid");
    (hand, bid)
}

fn part_one(file_name: &str) {
    let file = File::open(file_name).expect("Unable to open file");
    let result: u64 = std::io::BufReader::new(file)
        .lines()
        .map(|line| parse_line(&line.expect("Unable to get line")))
        .sorted_by(|(hand, _bid), (other, _other_bid)| {
            cmp_hands(hand, other, evaluate_hand_part_one, value_of_card_part_one)
        })
        .enumerate()
        .map(|(rank, (_hand, bid))| bid * (rank as u64 + 1))
        .sum();
    println!("{}", result);
}

fn part_two(file_name: &str) {
    let file = File::open(file_name).expect("Unable to open file");
    let result: u64 = std::io::BufReader::new(file)
        .lines()
        .map(|line| parse_line(&line.expect("Unable to get line")))
        .sorted_by(|(hand, _bid), (other, _other_bid)| {
            cmp_hands(hand, other, evaluate_hand_part_two, value_of_card_part_two)
        })
        .enumerate()
        .map(|(rank, (_hand, bid))| bid * (rank as u64 + 1))
        .sum();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one_example() {
        part_one("inputs/day_7/example.txt");
    }

    #[test]
    fn test_part_one() {
        part_one("inputs/day_7/input.txt");
    }

    #[test]
    fn test_part_two_example() {
        part_two("inputs/day_7/example.txt");
    }

    #[test]
    fn test_part_two() {
        part_two("inputs/day_7/input.txt");
    }

    #[rstest]
    #[case("AAAAA", HandType::FiveOfAKind)]
    #[case("AA8AA", HandType::FourOfAKind)]
    #[case("23332", HandType::FullHouse)]
    #[case("TTT98", HandType::ThreeOfAKind)]
    #[case("23432", HandType::TwoPair)]
    #[case("A23A4", HandType::Pair)]
    #[case("23456", HandType::HighCard)]
    fn test_hand_evaluation(#[case] hand: &str, #[case] expected_hand_type: HandType) {
        assert_eq!(evaluate_hand_part_one(hand), expected_hand_type);
    }

    #[test]
    fn test_hand_ranking() {
        let hands = vec!["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"];
        let mut sorted = hands.clone();
        sorted.sort_by(|hand, other| {
            cmp_hands(hand, other, evaluate_hand_part_one, value_of_card_part_one)
        });
        assert_eq!(hands, sorted);
    }

    #[rstest]
    #[case("AAAAJ", HandType::FiveOfAKind)]
    #[case("AA8AJ", HandType::FourOfAKind)]
    #[case("23J32", HandType::FullHouse)]
    #[case("TTJ98", HandType::ThreeOfAKind)]
    #[case("A23J4", HandType::Pair)]
    fn test_hand_evaluation_part_two(#[case] hand: &str, #[case] expected_hand_type: HandType) {
        assert_eq!(evaluate_hand_part_two(hand), expected_hand_type);
    }

    #[test]
    fn test_hand_ranking_part_two() {
        let hands = vec!["32T3K", "KK677", "T55J5", "QQQJA", "KTJJT"];
        let mut sorted = hands.clone();
        sorted.sort_by(|hand, other| {
            cmp_hands(hand, other, evaluate_hand_part_two, value_of_card_part_two)
        });
        assert_eq!(hands, sorted);
    }
}
