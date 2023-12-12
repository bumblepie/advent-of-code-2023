use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::ops::Add;
use std::str::FromStr;
use itertools::Itertools;
use lazy_regex::{regex, regex_captures};

fn parse_line(line: &str) -> (usize, HashSet<u64>, HashSet<u64>)
{
    let (_overall_match, card_number, winning_numbers, numbers) = regex_captures!(r#"Card\s+(\d+):\s+((?:\d+\s*)+)\|\s+((?:\d+\s*)+)"#, line)
        .expect("Unable to parse line");
    let card_number = usize::from_str(card_number).expect("Unable to parse card number");
    (card_number, parse_numbers(winning_numbers), parse_numbers(numbers))
}

fn parse_numbers(line: &str) -> HashSet<u64>
{
    let regex = regex!(r#"(\d+)\s*"#);
    regex.captures_iter(line)
        .map(|captures| u64::from_str(&captures[1]).expect("Unable to parse number"))
        .collect()
}

fn score_line(winning_numbers: &HashSet<u64>, numbers: &HashSet<u64>) -> u64
{
    let intersection_count = winning_numbers.intersection(numbers).count();
    if intersection_count < 1 {
        return 0;
    }
    return 2u64.pow(intersection_count as u32 - 1);
}

fn winners(card_id: usize, winning_numbers: &HashSet<u64>, numbers: &HashSet<u64>) -> Vec<usize>
{
    let intersection_count = winning_numbers.intersection(numbers).count();
    if intersection_count < 1 {
        return Vec::new();
    }
    return (1..(intersection_count + 1))
        .map(|index| card_id + index)
        .collect()
}

fn part_one()
{
    let file = File::open("../inputs/day_4/input.txt").expect("Unable to open file");
    let lines = std::io::BufReader::new(file).lines();

    let result: u64 = lines.map(|line| line.expect("Unable to parse line"))
        .map(|line| parse_line(line.as_str()))
        .map(|(_card_number, winning_numbers, numbers)| score_line(&winning_numbers, &numbers))
        .sum();
    println!("{}", result);
}

fn part_two()
{
    let file = File::open("inputs/day_4/input.txt").expect("Unable to open file");
    let lines: Vec<_> = std::io::BufReader::new(file).lines()
        .collect::<Result<_, _>>()
        .expect("Failed tor ead line from file");

    let mut card_scores: HashMap<usize, u64> = HashMap::new();
    for line in lines.iter().rev() {
        let (card_id, winning_numbers, numbers) = parse_line(line);
        let ticket_value: u64 = winners(card_id, &winning_numbers, &numbers).into_iter()
            .map(|card_id| card_scores[&card_id])
            .sum();
        card_scores.insert(card_id, ticket_value + 1);
    }
    println!("Total tickets value: {}", card_scores.values().sum::<u64>());
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part_one()
    {
        super::part_one();
    }

    #[test]
    fn part_two()
    {
        super::part_two();
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_line_scoring(#[case] line: &str, #[case] expected_score: u64)
    {
        let (_card_number, winning_numbers, numbers) = parse_line(line);
        assert_eq!(score_line(&winning_numbers, &numbers), expected_score);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", vec![2,3,4,5])]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", vec![3,4])]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", vec![4,5])]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", vec![5])]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", vec![])]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", vec![])]
    fn test_line_dependencies(#[case] line: &str, #[case] expected_dependencies: Vec<usize>)
    {
        let (card_number, winning_numbers, numbers) = parse_line(line);
        assert_eq!(winners(card_number, &winning_numbers, &numbers), expected_dependencies);
    }
}