use itertools::Itertools;
use lazy_regex::regex_find;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

fn find_first_digit(line: &str, allow_text: bool) -> Option<u64> {
    let digit = if allow_text {
        regex_find!(r#"\d|one|two|three|four|five|six|seven|eight|nine"#, line)?
    } else {
        regex_find!(r#"\d"#, line)?
    };
    return match digit {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => Some(u64::from_str(&digit).expect("Unable to parse digit")),
    };
}

fn find_last_digit(line: &str, allow_text: bool) -> Option<u64> {
    let reversed: String = line.chars().rev().collect();
    let digit = if allow_text {
        regex_find!(
            r#"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin"#,
            &reversed
        )?
    } else {
        regex_find!(r#"\d"#, &reversed)?
    };
    return match digit {
        "eno" => Some(1),
        "owt" => Some(2),
        "eerht" => Some(3),
        "ruof" => Some(4),
        "evif" => Some(5),
        "xis" => Some(6),
        "neves" => Some(7),
        "thgie" => Some(8),
        "enin" => Some(9),
        _ => Some(u64::from_str(&digit).expect("Unable to parse digit")),
    };
}

fn part_one() {
    let file = File::open("inputs/day1.txt").expect("Unable to open file");
    let lines = std::io::BufReader::new(file).lines();

    let result = lines
        .map_ok(|line| {
            10 * find_first_digit(&line, false).expect("first digit not found")
                + find_last_digit(&line, false).expect("last digit not found")
        })
        .fold_ok(0, |sum, next| sum + next)
        .expect("Error while reading file");
    println!("{}", result);
}

fn part_two() {
    let file = File::open("inputs/day1.txt").expect("Unable to open file");
    let lines = std::io::BufReader::new(file).lines();

    let result = lines
        .map_ok(|line| {
            10 * find_first_digit(&line, true).expect("first digit not found")
                + find_last_digit(&line, true).expect("last digit not found")
        })
        .fold_ok(0, |sum, next| sum + next)
        .expect("Error while reading file");
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part_one() {
        super::part_one();
    }

    #[test]
    fn part_two() {
        super::part_two();
    }

    #[rstest]
    #[case("1abc2", 1)]
    #[case("pqr3stu8vwx", 3)]
    #[case("a1b2c3d4e5f", 1)]
    #[case("treb7uchet", 7)]
    fn test_find_first_digit(#[case] line: &str, #[case] expected_result: u64) {
        assert_eq!(find_first_digit(line, false), Some(expected_result));
    }

    #[rstest]
    #[case("1abc2", 2)]
    #[case("pqr3stu8vwx", 8)]
    #[case("a1b2c3d4e5f", 5)]
    #[case("treb7uchet", 7)]
    fn test_find_last_digit(#[case] line: &str, #[case] expected_result: u64) {
        assert_eq!(find_last_digit(line, false), Some(expected_result));
    }

    #[rstest]
    #[case("two1nine", 2)]
    #[case("eightwothree", 8)]
    #[case("abcone2threexyz", 1)]
    #[case("xtwone3four", 2)]
    #[case("4nineeightseven2", 4)]
    #[case("zoneight234", 1)]
    #[case("7pqrstsixteen", 7)]
    fn test_find_first_digit_p2(#[case] line: &str, #[case] expected_result: u64) {
        assert_eq!(find_first_digit(line, true), Some(expected_result));
    }

    #[rstest]
    #[case("two1nine", 9)]
    #[case("eightwothree", 3)]
    #[case("abcone2threexyz", 3)]
    #[case("xtwone3four", 4)]
    #[case("4nineeightseven2", 2)]
    #[case("zoneight234", 4)]
    #[case("7pqrstsixteen", 6)]
    fn test_find_last_digit_p2(#[case] line: &str, #[case] expected_result: u64) {
        assert_eq!(find_last_digit(line, true), Some(expected_result));
    }
}
