use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use lazy_regex::{Lazy, regex};
use regex::Regex;
use itertools::Itertools;

fn parse_game(line: &str) -> HashMap<String, Vec<u64>> {
    let regex: &Lazy<Regex> = regex!(r#"(\d+)\s+(red|blue|green)"#);
    regex.captures_iter(line)
        .map(|captures| {
            let number = captures[1].to_string();
            let colour = captures[2].to_string();
            let number = u64::from_str(&number).expect(&format!("Unable to parse number {}", number));
            (colour, number)
        })
        .into_group_map()
}
fn p1_possible(game: &HashMap<String, Vec<u64>>) -> bool {
    *game["red"].iter().max().unwrap_or(&0) <= 12
    && *game["green"].iter().max().unwrap_or(&0) <= 13
    && *game["blue"].iter().max().unwrap_or(&0) <= 14
}

fn p2_power(game: &HashMap<String, Vec<u64>>) -> u64 {
    game["red"].iter().max().unwrap_or(&0) * game["green"].iter().max().unwrap_or(&0) * game["blue"].iter().max().unwrap_or(&0)
}

fn part_one() {
    let file = File::open("inputs/day_2.txt").expect("Unable to open file");
    let lines = std::io::BufReader::new(file).lines();

    let result: usize = lines
        .map(|line| line.unwrap_or_else(|_| panic!("Failed to read line")))
        .map(|line| parse_game(&line))
        .enumerate()
        .filter(|(_index, game)| p1_possible(&game))
        .map(|(index, _game)| index + 1)
        .sum();

    println!("{}", result);
}

fn part_two() {
    let file = File::open("inputs/day_2.txt").expect("Unable to open file");
    let lines = std::io::BufReader::new(file).lines();

    let result: u64 = lines
        .map(|line| line.unwrap_or_else(|_| panic!("Failed to read line")))
        .map(|line| parse_game(&line))
        .map(|game| p2_power(&game))
        .sum();

    println!("{}", result);
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
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", true)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", false)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", false)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_part_one(#[case] input: &str, #[case] expected_possible: bool)
    {
        let game = parse_game(input);
        assert_eq!(p1_possible(&game), expected_possible);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_part_two(#[case] input: &str, #[case] power: u64)
    {
        let game = parse_game(input);
        assert_eq!(p2_power(&game), power);
    }
}