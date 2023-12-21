use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use itertools::Itertools;

fn determine_next_value(sequence: Vec<i64>) -> i64
{
    if sequence.iter().all_equal() {
       return sequence[0];
    }

    let next_sequence = sequence.iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    let next_value_in_sequence_above = determine_next_value(next_sequence);
    return sequence.last().unwrap() + next_value_in_sequence_above;
}

fn determine_previous_value(sequence: Vec<i64>) -> i64
{
    if sequence.iter().all_equal() {
        return sequence[0];
    }

    let next_sequence = sequence.iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    let previous_value_in_sequence_above = determine_previous_value(next_sequence);
    return sequence.first().unwrap() - previous_value_in_sequence_above;
}

fn parse_line(line: &str) -> Vec<i64>
{
    line.split_whitespace()
        .map(|number| i64::from_str(number).expect("Failed to parse number"))
        .collect()
}

fn part_one(file_name: &str)
{
    let file = File::open(file_name).expect("Unable to open file");
    let result: i64 = std::io::BufReader::new(file).lines()
        .map(|line| parse_line(&line.expect("Unable to read line")))
        .map(|sequence| determine_next_value(sequence))
        .sum();
    println!("{}", result)
}

fn part_two(file_name: &str)
{
    let file = File::open(file_name).expect("Unable to open file");
    let result: i64 = std::io::BufReader::new(file).lines()
        .map(|line| parse_line(&line.expect("Unable to read line")))
        .map(|sequence| determine_previous_value(sequence))
        .sum();
    println!("{}", result)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one_example() {
        part_one("inputs/day_9/example.txt");
    }

    #[test]
    fn test_part_one() {
        part_one("inputs/day_9/input.txt");
    }

    #[test]
    fn test_part_two_example() {
        part_two("inputs/day_9/example.txt");
    }

    #[test]
    fn test_part_two() {
        part_two("inputs/day_9/input.txt");
    }

    #[rstest]
    #[case(vec![0, 3, 6, 9, 12, 15], 18)]
    #[case(vec![1, 3, 6, 10, 15, 21], 28)]
    #[case(vec![10, 13, 16, 21, 30, 45], 68)]
    fn test_determine_next_value(#[case] sequence: Vec<i64>, #[case] expected_result: i64)
    {
        assert_eq!(determine_next_value(sequence), expected_result);
    }

    #[rstest]
    #[case(vec![0, 3, 6, 9, 12, 15], -3)]
    #[case(vec![1, 3, 6, 10, 15, 21], 0)]
    #[case(vec![10, 13, 16, 21, 30, 45], 5)]
    fn test_determine_previous_value(#[case] sequence: Vec<i64>, #[case] expected_result: i64)
    {
        assert_eq!(determine_previous_value(sequence), expected_result);
    }
}