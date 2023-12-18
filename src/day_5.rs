use std::collections::HashMap;
use itertools::Itertools;
use lazy_regex::{regex, regex_captures};
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct ValueRange {
    inner: Range<i64>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Transformation {
    source_range: Range<i64>,
    offset: i64,
}

fn parse_seeds(line: &str) -> Vec<i64> {
    let regex = regex!(r#"(\d+)"#);
    regex
        .captures_iter(line)
        .map(|captures| i64::from_str(&captures[1]).expect("Unable to parse number"))
        .collect()
}

fn parse_transformation(line: &str) -> Transformation {
    let (_full_match, dest_range_start, source_range_start, range_length) =
        regex_captures!(r#"(\d+) (\d+) (\d+)"#, line).expect("Unable to parse mapping line");
    let source_range_start = i64::from_str(source_range_start).expect("Unable to parse number");
    let dest_range_start = i64::from_str(dest_range_start).expect("Unable to parse number");
    let range_length = i64::from_str(range_length).expect("Unable to parse number");

    Transformation {
        source_range: (source_range_start..(source_range_start + range_length)),
        offset: dest_range_start - source_range_start,
    }
}

fn parse_map(lines: Vec<&str>) -> ((&str, &str), Vec<Transformation>) {
    let (_full_match, from, to) = regex_captures!(r#"(\w+)\-to\-(\w+) map:"#, lines[0])
        .expect("Unable to parse mapping header");
    let transformations = lines[1..]
        .into_iter()
        .map(|line| parse_transformation(*line))
        .collect_vec();
    ((from, to), transformations)
}

fn parse_input(input: &str) -> (Vec<i64>, HashMap<(&str, &str), Vec<Transformation>>)
{
    let mut sections = input.split("\r\n\r\n");
    let seed_values = parse_seeds(sections.next().unwrap());
    let mappings = sections.map(|section| section.lines().collect_vec())
        .map(parse_map)
        .collect();
    (seed_values, mappings)
}

fn map_value(input: i64, mappings: &Vec<Transformation>) -> i64 {
    let mapping = mappings
        .iter()
        .filter(|mapping| mapping.source_range.contains(&input))
        .next();

    if let Some(mapping) = mapping {
        return input + mapping.offset;
    }

    return input;
}

fn map_values(input: ValueRange, mappings: &Vec<Transformation>) -> Vec<ValueRange> {
    unimplemented!()
}

fn part_one(file_name: &str) {
    let file_contents = std::fs::read_to_string(file_name)
        .expect("Unable to read file");
    let (seeds, mappings) = parse_input(file_contents.as_str());
    let lowest_location = seeds
        .into_iter()
        .map(|seed| map_value(seed, &mappings[&("seed", "soil")]))
        .map(|seed| map_value(seed, &mappings[&("soil", "fertilizer")]))
        .map(|seed| map_value(seed, &mappings[&("fertilizer", "water")]))
        .map(|seed| map_value(seed, &mappings[&("water", "light")]))
        .map(|seed| map_value(seed, &mappings[&("light", "temperature")]))
        .map(|seed| map_value(seed, &mappings[&("temperature", "humidity")]))
        .map(|seed| map_value(seed, &mappings[&("humidity", "location")]))
        .min()
        .unwrap();
    println!("{}", lowest_location);
}

mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part_one_example()
    {
        part_one("inputs/day_5/example.txt");
    }

    #[test]
    fn test_part_one()
    {
        part_one("inputs/day_5/input.txt");
    }


    #[rstest]
    #[case(0, 0)]
    #[case(9, 9)]
    #[case(10, 12)]
    #[case(19, 21)]
    #[case(20, 20)]
    #[case(30, 30)]
    fn test_map_single_value(#[case] input: i64, #[case] expected_output: i64) {
        let mappings = vec![Transformation {
            source_range: (10..20),
            offset: 2,
        }];
        assert_eq!(map_value(input, &mappings), expected_output);
    }

    #[test]
    fn test_all_map_to_one_big_range() {
        let seed_range = ValueRange { inner: (50..60) };
        let mappings = vec![Transformation {
            source_range: (0..100),
            offset: 12,
        }];
        assert_eq!(
            map_values(seed_range, &mappings),
            vec![ValueRange { inner: (62..72) }]
        );
    }
}
