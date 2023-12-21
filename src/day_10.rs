use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Pipe {
    Vertical,
    Horizontal,
    L90,
    J90,
    Seven90,
    F90,
}

impl Pipe {
    fn connections(&self) -> Vec<Point>
    {
        let above = Point {
            x: 0,
            y: -1,
        };
        let below = Point {
            x: 0,
            y: 1,
        };
        let left = Point {
            x: -1,
            y: 0,
        };
        let right = Point {
            x: 1,
            y: 0,
        };
        match self {
            Pipe::Vertical => vec![above, below],
            Pipe::Horizontal => vec![left, right],
            Pipe::L90 => vec![above, right],
            Pipe::J90 => vec![above, left],
            Pipe::Seven90 => vec![left, below],
            Pipe::F90 => vec![right, below],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn inverted(&self) -> Self {
        Self {
            x: self.x * -1,
            y: self.y * -1,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
#[derive(Debug, Clone)]
struct Marker {
    current: Point,
    next: Point,
}

#[derive(Debug, Clone)]
struct Map {
    pipes: HashMap<Point, Pipe>,
    starting_position: Point,
}

impl Map {
    fn longest_distance_from_start(&self) -> i64 {
        let mut markers: (_, _) = self.pipes[&self.starting_position].connections()
            .into_iter()
            .map(|connection| Marker {
                current: self.starting_position.clone(),
                next: self.starting_position.clone() + connection,
            })
            .collect_tuple()
            .unwrap();

        let mut steps = 0;
        loop {
            steps += 1;

            // Step Left
            let next_connection = self.pipes[&markers.0.next].connections()
                .into_iter()
                .map(|connection| markers.0.next.clone() + connection)
                .filter(|position| position != &markers.0.current)
                .exactly_one()
                .unwrap();
            // Check both locations in case we crossed-over
            if next_connection == markers.1.current || next_connection == markers.1.next {
                return steps;
            }
            markers.0 = Marker {
                next: next_connection,
                current: markers.0.next,
            };

            // Step Right
            let next_connection = self.pipes[&markers.1.next].connections()
                .into_iter()
                .map(|connection| markers.1.next.clone() + connection)
                .filter(|position| position != &markers.1.current)
                .exactly_one()
                .unwrap();
            if next_connection == markers.0.current {
                return steps;
            }
            markers.1 = Marker {
                next: next_connection,
                current: markers.1.next,
            };
        }
    }

    fn get_loop_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        let mut points_to_explore = vec![self.starting_position.clone()];

        while let Some(point) = points_to_explore.pop() {
            if points.contains(&point) {
                continue;
            }
            points.insert(point.clone());
            let connected_points = self.pipes[&point].connections()
                .into_iter()
                .map(|connection| point.clone() + connection)
                .filter(|point| !points.contains(point))
                .collect_vec();
            points_to_explore.extend(connected_points.into_iter());
        }
        points
    }

    fn complete_starting_position(&mut self)
    {
        let above = Point {
            x: 0,
            y: -1,
        };
        let below = Point {
            x: 0,
            y: 1,
        };
        let left = Point {
            x: -1,
            y: 0,
        };
        let right = Point {
            x: 1,
            y: 0,
        };
        let surrounding_connected_pipes = vec![above.clone(), below.clone(), left.clone(), right.clone()].into_iter()
            .filter(|relative| {
                let position = self.starting_position.clone() + relative.clone();
                if let Some(pipe) = self.pipes.get(&position) {
                    return pipe.connections().contains(&relative.inverted());
                }
                false
            })
            .collect_vec();

        let pipe = match (&surrounding_connected_pipes[0], &surrounding_connected_pipes[1]) {
            (x, y) if *x == above && *y == below => Pipe::Vertical,
            (x, y) if *x == left && *y == right => Pipe::Horizontal,
            (x, y) if *x == above && *y == left => Pipe::J90,
            (x, y) if *x == above && *y == right => Pipe::L90,
            (x, y) if *x == below && *y == left => Pipe::Seven90,
            (x, y) if *x == below && *y == right => Pipe::F90,
            _ => panic!("Unable to figure out starting pipes"),
        };
        self.pipes.insert(self.starting_position.clone(), pipe);
    }

    fn count_enclosed_points(&self) -> i64
    {
        let min_y = self.pipes.keys()
            .map(|p| p.y)
            .min()
            .unwrap();
        let max_y = self.pipes.keys()
            .map(|p| p.y)
            .max()
            .unwrap();
        dbg!(min_y..(max_y + 1))
            .map(|y| self.count_enclosed_points_in_row(y))
            .sum()
    }
    fn count_enclosed_points_in_row(&self, row: i64) -> i64
    {
        let pipes_in_row = self.pipes.keys()
            .filter(|point| point.y == row)
            .collect_vec();
        let min_x = pipes_in_row.iter()
            .map(|p| p.x)
            .min()
            .unwrap();
        let max_x = pipes_in_row.iter()
            .map(|p| p.x)
            .max()
            .unwrap();
        let loop_points = self.get_loop_points();

        let mut last_corner_seen = None;
        let mut inside_pipes = false;
        let mut count = 0;
        for x in min_x..(max_x + 1) {
            let point = Point{ x, y: row as i64 };
            if loop_points.contains(&point) {
                let pipe = self.pipes[&point].clone();
                match pipe {
                    Pipe::Vertical => inside_pipes = !inside_pipes,
                    Pipe::Horizontal => {},
                    Pipe::F90 => last_corner_seen = Some(Pipe::F90),
                    Pipe::L90 => last_corner_seen = Some(Pipe::L90),
                    Pipe::J90 => {
                        if last_corner_seen == Some(Pipe::F90) {
                            inside_pipes = !inside_pipes;
                        }
                        last_corner_seen = None;
                    },
                    Pipe::Seven90 => {
                        if last_corner_seen == Some(Pipe::L90) {
                            inside_pipes = !inside_pipes;
                        }
                        last_corner_seen = None;
                    }
                }
            } else {
                if inside_pipes {
                    count += 1;
                }
            }
        }

        count
    }
}

fn parse_input(file_name: &str) -> Map
{
    let file = File::open(file_name).expect("Unable to open file");
    let lines = std::io::BufReader::new(file).lines();
    let mut pipes = HashMap::new();
    let mut starting_position = None;
    for (row, line) in lines.enumerate() {
        let line = line.expect("Unable to read line");
        for (column, character) in line.chars().enumerate() {
            let position = Point {
                x: column as i64,
                y: row as i64,
            };
            if character == 'S' {
                starting_position = Some(position.clone());
            }
            let pipe = match character {
                '|' => Some(Pipe::Vertical),
                '-' => Some(Pipe::Horizontal),
                'L' => Some(Pipe::L90),
                'J' => Some(Pipe::J90),
                '7' => Some(Pipe::Seven90),
                'F' => Some(Pipe::F90),
                'S' => None,
                '.' => None,
                _ => panic!("Unexpected character in input"),
            };
            if let Some(pipe) = pipe {
                pipes.insert(position, pipe);
            }
        }
    }
    let mut result = Map {
        pipes,
        starting_position: starting_position.expect("Unable to find starting position")
    };
    result.complete_starting_position();
    result
}

fn part_one(file_name: &str) -> i64
{
    let map = parse_input(file_name);
    map.longest_distance_from_start()
}
fn part_two(file_name: &str) -> i64
{
    let map = parse_input(file_name);
    map.count_enclosed_points()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example()
    {
        let result = part_one("inputs/day_10/example.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_one()
    {
        let result = part_one("inputs/day_10/input.txt");
        println!("{}", result);
    }

    #[test]
    fn test_example_2_part_two()
    {
        let result = part_two("inputs/day_10/example_2.txt");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_3_part_two()
    {
        let result = part_two("inputs/day_10/example_3.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_example_4_part_two()
    {
        let result = part_two("inputs/day_10/example_4.txt");
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part_two()
    {
        let result = part_two("inputs/day_10/input.txt");
        println!("{}", result);
    }
}