// dist = v * t
// v = time_spent_holding_button
// t = total_time - time_spent_holding_button
// dist = time_spent_holding_button * (total_time - time_spent_holding_button)
// dist = time_spent_holding_button * total_time - time_spent_holding_button^2
// T^2 - T*TT + dist = 0
// a = 1
// b = (-1 * total time)
// c = dist
// x = (-b +/- sqrt(b^2 -4ac)) / 2a
// eg: race(7, 9) -> (7 +/- sqrt(7^2 -4*1*9) / (2*1)
// eg: (7 +/- sqrt(13)) / 2
// eg: (7 +/- 3.6...) / 2
// eg: (3.6... or 10.6...) / 2
// eg: 1.8... or 5.3...
// eg: 2 -> 5 = 4 values

use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

fn num_winnable_values_for_race(time: f64, record: f64) -> u64 {
    let sqrt_b_squared_minus_4ac = ((time * time) - (4.0 * record)).sqrt();
    let min_winning_value = ((time - sqrt_b_squared_minus_4ac) / 2.0).floor() as u64 + 1;
    let max_winning_value = ((time + sqrt_b_squared_minus_4ac) / 2.0).ceil() as u64 - 1;
    max_winning_value + 1 - min_winning_value
}

fn parse_input(file_name: &str) -> Vec<(f64, f64)> {
    let file = File::open(file_name).expect("Unable to open file");
    let mut lines = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"));
    let races = lines
        .next()
        .expect("Could not fetch race times from input")
        .split_whitespace()
        .skip(1)
        .map(|time| f64::from_str(time).expect("Unable to parse time"))
        .collect_vec();
    let records = lines
        .next()
        .expect("Could not fetch records from input")
        .split_whitespace()
        .skip(1)
        .map(|record| f64::from_str(record).expect("Unable to parse record"))
        .collect_vec();

    races.into_iter().zip(records).collect()
}

fn parse_input_part_two(file_name: &str) -> (f64, f64) {
    let file = File::open(file_name).expect("Unable to open file");
    let mut lines = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"));
    let time = lines
        .next()
        .expect("Could not fetch race times from input")
        .split_whitespace()
        .skip(1)
        .join("");
    let time = f64::from_str(&time).expect("Unable to parse time");
    let record = lines
        .next()
        .expect("Could not fetch records from input")
        .split_whitespace()
        .skip(1)
        .join("");
    let record = f64::from_str(&record).expect("Unable to parse record");

    (time, record)
}

fn part_one(file_name: &str) {
    let races = parse_input(file_name);
    let result = races
        .into_iter()
        .map(|(time, record)| num_winnable_values_for_race(time, record))
        .fold(1, |current, next| current * next);
    println!("{}", result);
}

fn part_two(file_name: &str) {
    let (time, record) = parse_input_part_two(file_name);
    let result = num_winnable_values_for_race(time, record);
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one_example() {
        part_one("inputs/day_6/example.txt");
    }

    #[test]
    fn test_part_one() {
        part_one("inputs/day_6/input.txt");
    }

    #[test]
    fn test_part_two_example() {
        part_two("inputs/day_6/example.txt");
    }

    #[test]
    fn test_part_two() {
        part_two("inputs/day_6/input.txt");
    }

    #[rstest]
    #[case(7.0, 9.0, 4)]
    #[case(15.0, 40.0, 8)]
    #[case(30.0, 200.0, 9)]
    fn test_winnable_values(#[case] time: f64, #[case] record: f64, #[case] expected_count: u64) {
        assert_eq!(num_winnable_values_for_race(time, record), expected_count);
    }
}
