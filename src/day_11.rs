use std::collections::HashSet;
use std::io::BufRead;
use itertools::Itertools;
use crate::util::Point;

struct Map {
    points: HashSet<Point>,
    empty_rows: HashSet<i64>,
    empty_columns: HashSet<i64>
}

fn parse_line(line: &str, row: usize) -> Vec<Point>
{
    line.chars()
        .enumerate()
        .filter(|(_column, c)| *c == '#')
        .map(|(column, _c)| Point { x: column as i64, y: row as i64})
        .collect()
}

fn parse_input(file_name: &str) -> Map
{
    let file = std::fs::File::open(file_name).expect("Unable to open file");
    let lines = std::io::BufReader::new(file).lines();
    let points: HashSet<_> = lines.map(|line| line.expect("Unable to read line"))
        .enumerate()
        .flat_map(|(row, line)| parse_line(&line, row))
        .collect();
    let columns: HashSet<_> = points.iter().map(|p| p.x).collect();
    let rows: HashSet<_> = points.iter().map(|p| p.y).collect();
    let column_min = *columns.iter().min().unwrap();
    let column_max = *columns.iter().max().unwrap();
    let empty_columns = (column_min..column_max)
        .filter(|x| !columns.contains(x))
        .collect();
    let rows_min = *rows.iter().min().unwrap();
    let rows_max = *rows.iter().max().unwrap();
    let empty_rows = (rows_min..rows_max)
        .filter(|y| !rows.contains(y))
        .collect();
    Map {
        empty_columns,
        empty_rows,
        points,
    }
}

fn part_one(file_name: &str) -> i64
{
    let map = parse_input(file_name);
    map.points.iter()
        .combinations(2)
        .map(|points| {
            let base_difference = points[0].clone() - points[1].clone();
            let empty_rows_crossed = (i64::min(points[0].y, points[1].y)..i64::max(points[0].y, points[1].y))
                .filter(|row| map.empty_rows.contains(row))
                .count() as i64;
            let empty_columns_crossed = (i64::min(points[0].x, points[1].x)..i64::max(points[0].x, points[1].x))
                .filter(|column| map.empty_columns.contains(column))
                .count() as i64;
            i64::abs(base_difference.x) + i64::abs(base_difference.y) + empty_rows_crossed + empty_columns_crossed
        })
        .sum()
}

fn part_two(file_name: &str, multiplier: i64) -> i64
{
    let map = parse_input(file_name);
    map.points.iter()
        .combinations(2)
        .map(|points| {
            let base_difference = points[0].clone() - points[1].clone();
            let empty_rows_crossed = (i64::min(points[0].y, points[1].y)..i64::max(points[0].y, points[1].y))
                .filter(|row| map.empty_rows.contains(row))
                .count() as i64;
            let empty_columns_crossed = (i64::min(points[0].x, points[1].x)..i64::max(points[0].x, points[1].x))
                .filter(|column| map.empty_columns.contains(column))
                .count() as i64;
            i64::abs(base_difference.x) + i64::abs(base_difference.y) + empty_rows_crossed * (multiplier - 1) + empty_columns_crossed * (multiplier - 1)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example()
    {
        let result = part_one("inputs/day_11/example.txt");
        assert_eq!(result, 374);
    }

    #[test]
    fn test_part_one()
    {
        let result = part_one("inputs/day_11/input.txt");
        println!("{}", result);
    }

    #[test]
    fn test_example_part_two()
    {
        let result = part_two("inputs/day_11/example.txt", 10);
        assert_eq!(result, 1030);
        let result = part_two("inputs/day_11/example.txt", 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn test_part_one_part_two()
    {
        let result = part_two("inputs/day_11/input.txt", 1_000_000);
        println!("{}", result);
    }
}
