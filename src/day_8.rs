use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use lazy_regex::regex_captures;

#[derive(Debug, Clone)]
struct Map {
    directions: String,
    nodes: HashMap<String, NodeConnections>
}

#[derive(Debug, Clone)]
struct NodeConnections {
    left: String,
    right: String,
}

fn parse_node(line: &str) -> (String, NodeConnections)
{
    let (_full_match, node, left, right) = regex_captures!(r#"(\w{3}) = \((\w{3}), (\w{3})\)"#, line)
        .expect("Unable to parse node");
    (node.to_string(), NodeConnections { left: left.to_string(), right: right.to_string() })
}

fn parse_input(contents: &str) -> Map
{
    let sections = contents.split("\n\n")
        .collect_vec();
    let directions = sections[0].trim().to_string();
    let nodes = sections[1].lines()
        .map(parse_node)
        .collect();
    Map {
        directions,
        nodes,
    }
}

fn part_one(file_name: &str) -> u64
{
    let file_contents = std::fs::read_to_string(file_name)
        .expect("Unable to read file")
        .replace("\r\n", "\n");
    let map = parse_input(&file_contents);
    let mut steps = 0;
    let mut location = "AAA".to_string();
    let mut directions = map.directions.chars().cycle();
    while location != "ZZZ" {
        steps +=1;
        let direction = directions.next().unwrap();
        location = match direction {
            'L' => map.nodes[&location].left.clone(),
            'R' => map.nodes[&location].right.clone(),
            _ => panic!("Unknown direction"),
        }
    }
    return steps;
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Cycle {
    offset: i64,
    length: i64,
}

impl Cycle {
    fn merge(self, other: Self) -> Self {
        if self.length > other.length {
            // Ensure self is always the cycle with the lower length
            return other.merge(self);
        }
        let mut desired_offset = self.offset - other.offset;
        while desired_offset < 0 {
            desired_offset += self.length;
        }

        let length_diff = other.length - self.length;
        let mut cycles_needed_to_sync = 0;
        let mut diff = cycles_needed_to_sync * length_diff;
        while !(diff % self.length == desired_offset % self.length && diff >= desired_offset) {
            cycles_needed_to_sync += 1;
            diff = cycles_needed_to_sync * length_diff;
        }
        let combined_offset = other.offset + other.length * cycles_needed_to_sync;
        let combined_length = num::integer::lcm(self.length, other.length);

        Cycle {
            offset: combined_offset,
            length: combined_length,
        }
    }
}

fn find_cycle(starting_node: &str, map: &Map) -> Cycle
{
    let mut explored_nodes = HashMap::new();
    let mut location = starting_node.to_string();
    let mut directions = map.directions.chars()
        .enumerate()
        .cycle();
    let (mut index, mut direction) = directions.next().unwrap();
    let mut z_nodes = HashSet::new();
    let mut steps = 0;
    while !explored_nodes.contains_key(&(location.clone(), index)) {
        if location.ends_with("Z") {
            z_nodes.insert(steps);
        }
        explored_nodes.insert((location.clone(), index), steps);
        location = match direction {
            'L' => map.nodes[&location].left.clone(),
            'R' => map.nodes[&location].right.clone(),
            _ => panic!("Unknown direction"),
        };
        steps += 1;
        (index, direction) = directions.next().unwrap();
    }
    let cycle_start = explored_nodes[&(location.clone(), index)];
    let first_z_node = *z_nodes.iter().min().unwrap();

    // Assumption: Z nodes are spaced out evenly
    // Holds true for the example which has multiple z nodes for the 22A path
    // May not hold true for all inputs, but my input only had paths with a single Z node each
    let cycle_length = (steps - cycle_start) / z_nodes.len();
    Cycle {
        offset: first_z_node as i64,
        length: cycle_length as i64,
    }
}

fn part_two(file_name: &str) -> i64
{
    let file_contents = std::fs::read_to_string(file_name)
        .expect("Unable to read file")
        .replace("\r\n", "\n");
    let map = parse_input(&file_contents);

    let full_cycle = map.nodes.keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| find_cycle(node, &map))
        .reduce(|cycle, other| cycle.merge(other));
    full_cycle.unwrap().offset
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one("inputs/day_8/example_1.txt");
        println!("{}", result);
    }

    #[test]
    fn test_part_one_example_2() {
        let result = part_one("inputs/day_8/example_2.txt");
        println!("{}", result);
    }

    #[test]
    fn test_part_one() {
        let result = part_one("inputs/day_8/input.txt");
        println!("{}", result);
    }

    #[test]
    fn test_part_two_example_3() {
        let result = part_two("inputs/day_8/example_3.txt");
        println!("{}", result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("inputs/day_8/input.txt");
        println!("{}", result);
    }

    #[test]
    fn test_find_cycle()
    {
        let input = std::fs::read_to_string("inputs/day_8/example_3.txt")
            .expect("Unable to read file")
            .replace("\r\n", "\n");
        let map = parse_input(&input);
        let result = find_cycle("11A", &map);
        assert_eq!(result, Cycle { offset: 2, length: 2});
        let result = find_cycle("22A", &map);
        assert_eq!(result, Cycle { offset: 3, length: 3 });
    }

    #[test]
    fn test_cycle_merge()
    {
        let cycle_1 = Cycle {
            offset: 4,
            length: 2,
        };
        let cycle_2 = Cycle {
            offset: 2,
            length: 3,
        };
        assert_eq!(cycle_1.clone().merge(cycle_2.clone()), Cycle { offset: 8, length: 6});
        assert_eq!(cycle_2.merge(cycle_1), Cycle { offset: 8, length: 6});
    }

    #[test]
    fn test_cycle_merge_negative_offset()
    {
        let cycle_1 = Cycle {
            offset: 2,
            length: 3,
        };
        let cycle_2 = Cycle {
            offset: 4,
            length: 5,
        };
        assert_eq!(cycle_1.clone().merge(cycle_2.clone()), Cycle { offset: 14, length: 15});
        assert_eq!(cycle_2.merge(cycle_1), Cycle { offset: 14, length: 15});
    }

    #[test]
    fn test_cycle_merge_large_offset()
    {
        let cycle_1 = Cycle {
            offset: 2,
            length: 3,
        };
        let cycle_2 = Cycle {
            offset: 20_001,
            length: 5,
        };
        assert_eq!(cycle_1.clone().merge(cycle_2.clone()), Cycle { offset: 20_006, length: 15});
        assert_eq!(cycle_2.merge(cycle_1), Cycle { offset: 20_006, length: 15});
    }
}