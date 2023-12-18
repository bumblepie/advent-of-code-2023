use itertools::Itertools;
use lazy_regex::{regex, Lazy};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct NumberPosition {
    row: usize,
    column_span: (usize, usize),
}

impl NumberPosition {
    fn adjacent(&self, symbol_position: &Point) -> bool {
        let column_values = self.column_span.0..(self.column_span.1);
        return i64::abs(self.row as i64 - symbol_position.row as i64) < 2
            && column_values
                .into_iter()
                .any(|column| i64::abs(column as i64 - symbol_position.column as i64) < 2);
    }
}

#[derive(Debug)]
struct Schematic {
    symbols: HashMap<Point, char>,
    numbers: HashMap<NumberPosition, u64>,
}

impl Schematic {
    fn new() -> Self {
        Schematic {
            numbers: HashMap::new(),
            symbols: HashMap::new(),
        }
    }

    fn gears(&self) -> Vec<u64> {
        self.symbols
            .iter()
            .filter(|(_position, symbol)| **symbol == '*')
            .map(|(position, _symbol)| {
                self.numbers
                    .iter()
                    .filter(|(number_position, _number)| number_position.adjacent(position))
                    .map(|(_number_position, number)| number)
                    .collect_vec()
            })
            .filter(|adjacent_numbers| adjacent_numbers.len() == 2)
            .map(|adjacent_numbers| adjacent_numbers[0] * adjacent_numbers[1])
            .collect()
    }
}

fn parse_line(
    line_number: usize,
    line: &str,
) -> (HashMap<NumberPosition, u64>, HashMap<Point, char>) {
    let numbers_regex: &Lazy<Regex> = regex!(r#"\d+"#);
    let numbers = numbers_regex
        .find_iter(line)
        .map(|m| {
            let range = m.range();
            let number =
                u64::from_str(m.as_str()).expect(&format!("Unable to parse number {}", m.as_str()));
            let number_position = NumberPosition {
                row: line_number,
                column_span: (range.start, range.end),
            };
            (number_position, number)
        })
        .collect();
    let symbols_regex: &Lazy<Regex> = regex!(r#"[^\d\.]"#);
    let symbols = symbols_regex
        .find_iter(line)
        .map(|m| {
            // Assumption: regex should match exactly one character -> match should be exactly one character long
            let position = Point {
                row: line_number,
                column: m.range().start,
            };
            let symbol = m.as_str().chars().next().unwrap();
            (position, symbol)
        })
        .collect();
    (numbers, symbols)
}

fn parse_input(lines: impl Iterator<Item = String>) -> Schematic {
    lines
        .enumerate()
        .map(|(line_number, line)| parse_line(line_number, line.as_str()))
        .fold(
            Schematic::new(),
            |mut schematic, (next_numbers, next_symbols)| {
                schematic.numbers.extend(next_numbers);
                schematic.symbols.extend(next_symbols);
                schematic
            },
        )
}

fn part_one(file_name: &str) -> u64 {
    let file = File::open(file_name).expect("Unable to open file");
    let lines = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"));

    let schematic = parse_input(lines);
    schematic
        .numbers
        .iter()
        .filter(|(number_position, _number)| {
            schematic
                .symbols
                .keys()
                .any(|symbol_point| number_position.adjacent(symbol_point))
        })
        .map(|(_number_position, number)| number)
        .sum()
}

fn part_two(file_name: &str) -> u64 {
    let file = File::open(file_name).expect("Unable to open file");
    let lines = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"));

    let schematic = parse_input(lines);
    schematic.gears().iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use std::io::BufRead;

    #[test]
    fn part_one() {
        println!("{}", super::part_one("inputs/day_3/input.txt"));
    }

    #[test]
    fn part_two() {
        println!("{}", super::part_two("inputs/day_3/input.txt"));
    }

    #[rstest]
    #[case(NumberPosition { row: 5, column_span: (4, 6) }, Point { row: 4, column: 4 }, true)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 4, column: 5}, true)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 4, column: 6}, true)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 4, column: 4}, true)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 4, column: 6}, true)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 5, column: 4}, true)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 5, column: 5}, true)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 5, column: 6}, true)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 3, column: 5}, false)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 7, column: 5}, false)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 4, column: 2}, false)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 4, column: 7}, false)]
    #[case(NumberPosition { row: 5, column_span: (4, 6) },  Point { row: 3, column: 3}, false)]
    fn test_number_position_adjacency(
        #[case] number_position: NumberPosition,
        #[case] symbol_position: Point,
        #[case] expected_result: bool,
    ) {
        assert_eq!(number_position.adjacent(&symbol_position), expected_result);
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(super::part_one("inputs/day_3/example.txt"), 4361);
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(super::part_two("inputs/day_3/example.txt"), 467835);
    }
}
