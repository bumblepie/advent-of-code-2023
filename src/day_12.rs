use std::io::BufRead;
use std::ops::Index;
use std::str::FromStr;
use cached::proc_macro::cached;
use itertools::Itertools;
use lazy_regex::{regex_captures};

#[cached]
fn possible_arrangements(line: String, mut arrangement: Vec<usize>) -> usize
{
    if line.len() < arrangement.iter().sum() {
        return 0;
    }
    if arrangement.len() == 0 {
        if line.contains("#") {
            // Unexpected springs
            return 0;
        }
        return 1
    }

    let middle_group_size = arrangement.remove(arrangement.len() / 2);
    // +1 to ensure we're splitting at the same place
    let (arrangement_head, arrangement_tail) = arrangement.split_at((arrangement.len() + 1) / 2);
    let arrangement_head = arrangement_head.to_vec();
    let arrangement_tail = arrangement_tail.to_vec();

    let possible_spans = find_possible_spring_spans(line.clone(), middle_group_size);
    let line_string = line.to_string();
    possible_spans.into_iter()
        .map(|(start, end)| {
            let line_head = line_string[..start].to_string() + ".";
            let line_tail = ".".to_string() + &line_string[end..];
            let head_arrangements = possible_arrangements(line_head.clone(), arrangement_head.clone());
            if head_arrangements == 0 {
                return 0;
            }
            let tail_arrangements = possible_arrangements(line_tail.clone(), arrangement_tail.clone());
            head_arrangements * tail_arrangements
        }).sum()
}

#[cached]
fn find_possible_spring_spans(line: String, len: usize) -> Vec<(usize, usize)>
{
    if (len + 2) > line.len() {
        return Vec::new();
    }
    let mut result = Vec::new();
    for start_index in 0..(line.len() - len - 1) {
        let slice = &line[start_index..(start_index + len + 2)].chars().collect_vec();
        if !(slice[0] == '.' || slice[0] == '?') {
            continue;
        }
        if !(slice[len + 1] == '.' || slice[len + 1] == '?') {
            continue;
        }
        let middle = &slice[1..(len + 1)];
        if !middle.iter().all(|c| *c == '#' || *c == '?') {
            continue;
        }
        result.push((start_index, start_index + len + 2));
    }
    return result;
}

fn parse_line(line: &str) -> (String, Vec<usize>)
{
    let (_full_match, line, arrangement) = regex_captures!(r#"([?#.]+) ([\d,]+)"#, line)
        .expect("Unable top parse line");
    let arrangement = arrangement.split(",")
        .map(|number| usize::from_str(number).expect("Unable to parse number"))
        .collect_vec();
    (line.to_string(), arrangement)
}

fn parse_line_part_two(line: &str) -> (String, Vec<usize>)
{
    let (_full_match, line, arrangement) = regex_captures!(r#"([?#.]+) ([\d,]+)"#, line)
        .expect("Unable top parse line");
    let arrangement = arrangement.split(",")
        .map(|number| usize::from_str(number).expect("Unable to parse number"))
        .collect_vec();
    let line = itertools::repeat_n(line, 5)
        .join("?");
    (line, arrangement.repeat(5))
}

fn part_one(file_name: &str) -> usize
{
    let file = std::fs::File::open(file_name).expect("Unable to open file");
    let lines = std::io::BufReader::new(file).lines();
    lines.map(|line| line.expect("Unable to get line"))
        .map(|line| parse_line(&line))
        .map(|(line, arrangement)| possible_arrangements(format!(".{}.", line), arrangement))
        .sum()
}

fn part_two(file_name: &str) -> usize
{
    let file = std::fs::File::open(file_name).expect("Unable to open file");
    let lines = std::io::BufReader::new(file).lines();
    lines.map(|line| line.expect("Unable to get line"))
        .map(|line| parse_line_part_two(&line))
        .map(|(line, arrangement)| possible_arrangements(format!(".{}.", line), arrangement))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("???.###", vec![1,1,3], 1)]
    #[case(".??..??...?##.", vec![1,1,3], 4)]
    #[case("?#?#?#?#?#?#?#?", vec![1,3,1,6], 1)]
    #[case("????.#...#...", vec![4,1,1], 1)]
    #[case("????.######..#####.", vec![1,6,5], 4)]
    #[case("?###????????", vec![3, 2, 1], 10)]
    #[case(".###??????????###.", vec![2, 1, 3], 0)]
    fn test_examples(#[case] line: &str, #[case] arrangement: Vec<usize>, #[case] expected_result: usize)
    {
        assert_eq!(possible_arrangements(format!(".{}.", line), arrangement), expected_result)
    }

    #[rstest]
    #[case("???.###", 3, 2)]
    #[case("?#?", 2, 2)]
    #[case("?????", 2, 4)]
    fn test_find_possible_spans(#[case] line: &str, #[case] len: usize, #[case] expected_result: usize)
    {
        assert_eq!(find_possible_spring_spans(format!(".{}.", line), len).len(), expected_result)
    }

    #[test]
    fn test_example()
    {
        let result = part_one("inputs/day_12/example.txt");
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_one()
    {
        let result = part_one("inputs/day_12/input.txt");
        println!("{}", result);
    }

    #[rstest]
    #[case("???.###", vec![1,1,3], 1)]
    #[case(".??..??...?##.", vec![1,1,3], 16384)]
    #[case("?#?#?#?#?#?#?#?", vec![1,3,1,6], 1)]
    #[case("????.#...#...", vec![4,1,1], 16)]
    #[case("????.######..#####.", vec![1,6,5], 2500)]
    #[case("?###????????", vec![3, 2, 1], 506250)]
    fn test_examples_part_two(#[case] line: &str, #[case] arrangement: Vec<usize>, #[case] expected_result: usize)
    {
        let line = itertools::repeat_n(line, 5)
            .join("?");
        let arrangement = arrangement.repeat(5);
        assert_eq!(possible_arrangements(format!(".{}.", line), arrangement), expected_result)
    }

    #[test]
    fn test_example_part_two()
    {
        let result = part_two("inputs/day_12/example.txt");
        assert_eq!(result, 525152);
    }

    #[test]
    fn test_part_two()
    {
        let result = part_two("inputs/day_12/input.txt");
        println!("{}", result);
    }
}