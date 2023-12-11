use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left = 0,
    Right = 1,
}

type Node = &'static str;

type Map = HashMap<Node, [Node; 2]>;

type Instruction = Vec<Direction>;

fn parse_input(input: &'static str) -> (Instruction, Map) {
    let mut parts = input.split("\n\n");
    let instruction = parts.next().unwrap().trim().chars().map(|c| {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }).collect::<Vec<_>>();
    let map = parts.next().unwrap().trim().lines().map(|line| {
        let mut parts = line.split(" = ");
        let node = parts.next().unwrap().trim();
        let mut children = parts.next().unwrap().split(", ");
        let left = children.next().unwrap().trim().strip_prefix("(").unwrap();
        let right = children.next().unwrap().trim().strip_suffix(")").unwrap();
        (node, [left, right])
    }).collect::<HashMap<_, _>>();
    (instruction, map)
}

const INPUT: &str = include_str!("input.txt");

fn num_steps(instruction: &Instruction, map: &Map, start: Node, end: Node) -> Option<usize> {
    let mut current = start;
    let mut result = 0;
    let mut history = HashMap::<Node, Vec<usize>>::new();
    loop {
        history.entry(current).or_default().push(result);
        let direction = instruction[result % instruction.len()];
        let next = map[current][direction as usize];
        result += 1;
        if next == end {
            return Some(result);
        }
        if let Some(history_positions) = history.get(&next) {
            if history_positions.iter().any(|position| (result - position) % instruction.len() == 0) {
                return None;
            }
        }
        current = next;
    }
}

fn main() {
    let (instruction, map) = parse_input(INPUT);
    let starts = map.keys().filter(|node| node.ends_with("A")).collect::<Vec<_>>();
    let ends = map.keys().filter(|node| node.ends_with("Z")).collect::<Vec<_>>();
    let possible_steps = starts.iter().map(|start| {
        ends.iter().flat_map(|end| {
            num_steps(&instruction, &map, start, end)
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    // Interesting...
    assert!(possible_steps.iter().all(|steps| steps.len() == 1));
    let steps = possible_steps.into_iter().flat_map(IntoIterator::into_iter).collect::<Vec<_>>();
    let result = steps.into_iter().reduce(num::integer::lcm).unwrap();
    println!("{}", result);
}
