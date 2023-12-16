use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum OccupanceKind {
    Empty,
    HorizontalSplit,
    VerticalSplit,
    LeftBottomMirror,
    RightBottomMirror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Occupance {
    kind: OccupanceKind,
    energized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Light {
    outer: usize,
    inner: usize,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Puzzle {
    grid: Vec<Vec<Occupance>>,
}

impl Puzzle {
    fn len_outer(&self) -> usize {
        self.grid.len()
    }

    fn len_inner(&self) -> usize {
        self.grid[0].len()
    }
}

fn left(light: Light, puzzle: &Puzzle) -> Option<Light> {
    if light.inner > 0 {
        Some(Light {
            outer: light.outer,
            inner: light.inner - 1,
            direction: Direction::Left,
        })
    } else {
        None
    }
}

fn right(light: Light, puzzle: &Puzzle) -> Option<Light> {
    if light.inner < puzzle.len_inner() - 1 {
        Some(Light {
            outer: light.outer,
            inner: light.inner + 1,
            direction: Direction::Right,
        })
    } else {
        None
    }
}

fn up(light: Light, puzzle: &Puzzle) -> Option<Light> {
    if light.outer > 0 {
        Some(Light {
            outer: light.outer - 1,
            inner: light.inner,
            direction: Direction::Up,
        })
    } else {
        None
    }
}

fn down(light: Light, puzzle: &Puzzle) -> Option<Light> {
    if light.outer < puzzle.len_outer() - 1 {
        Some(Light {
            outer: light.outer + 1,
            inner: light.inner,
            direction: Direction::Down,
        })
    } else {
        None
    }
}

fn next_light(light: Light, puzzle: &Puzzle) -> Vec<Light> {
    match (light.direction, puzzle.grid[light.outer][light.inner].kind) {
        (Direction::Up, OccupanceKind::Empty | OccupanceKind::VerticalSplit) => {
            up(light, puzzle).into_iter().collect()
        }
        (Direction::Down, OccupanceKind::Empty | OccupanceKind::VerticalSplit) => {
            down(light, puzzle).into_iter().collect()
        }
        (Direction::Left, OccupanceKind::Empty | OccupanceKind::HorizontalSplit) => {
            left(light, puzzle).into_iter().collect()
        }
        (Direction::Right, OccupanceKind::Empty | OccupanceKind::HorizontalSplit) => {
            right(light, puzzle).into_iter().collect()
        }
        (Direction::Up | Direction::Down, OccupanceKind::HorizontalSplit) => {
            [left(light, puzzle), right(light, puzzle)]
                .into_iter()
                .flatten()
                .collect()
        }
        (Direction::Left | Direction::Right, OccupanceKind::VerticalSplit) => {
            [up(light, puzzle), down(light, puzzle)]
                .into_iter()
                .flatten()
                .collect()
        }
        (Direction::Up, OccupanceKind::LeftBottomMirror) => {
            left(light, puzzle).into_iter().collect()
        }
        (Direction::Up, OccupanceKind::RightBottomMirror) => {
            right(light, puzzle).into_iter().collect()
        }
        (Direction::Down, OccupanceKind::LeftBottomMirror) => {
            right(light, puzzle).into_iter().collect()
        }
        (Direction::Down, OccupanceKind::RightBottomMirror) => {
            left(light, puzzle).into_iter().collect()
        }
        (Direction::Left, OccupanceKind::LeftBottomMirror) => {
            up(light, puzzle).into_iter().collect()
        }
        (Direction::Left, OccupanceKind::RightBottomMirror) => {
            down(light, puzzle).into_iter().collect()
        }
        (Direction::Right, OccupanceKind::LeftBottomMirror) => {
            down(light, puzzle).into_iter().collect()
        }
        (Direction::Right, OccupanceKind::RightBottomMirror) => {
            up(light, puzzle).into_iter().collect()
        }
    }
}

fn count_energized(mut puzzle: Puzzle, light: Light) -> usize {
    let mut lights = vec![light];
    let mut history = HashSet::new();
    while let Some(light) = lights.pop() {
        if !history.contains(&light) {
            history.insert(light);
            puzzle.grid[light.outer][light.inner].energized = true;
            lights.extend(next_light(light, &puzzle));
        }
    }

    puzzle.grid.iter().flatten().filter(|o| o.energized).count()
}

fn main() {
    let puzzle = Puzzle {
        grid: INPUT
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Occupance {
                        kind: match c {
                            '.' => OccupanceKind::Empty,
                            '|' => OccupanceKind::VerticalSplit,
                            '-' => OccupanceKind::HorizontalSplit,
                            '/' => OccupanceKind::RightBottomMirror,
                            '\\' => OccupanceKind::LeftBottomMirror,
                            _ => panic!("Unknown character: {}", c),
                        },
                        energized: false,
                    })
                    .collect()
            })
            .collect(),
    };
    let mut result = 0;
    // Top row
    for inner in 0..puzzle.len_inner() {
        result = result.max(count_energized(puzzle.clone(), Light {
            outer: 0,
            inner,
            direction: Direction::Down,
        }));
    }
    // Bottom row
    for inner in 0..puzzle.len_inner() {
        result = result.max(count_energized(puzzle.clone(), Light {
            outer: puzzle.len_outer() - 1,
            inner,
            direction: Direction::Up,
        }));
    }
    // Left column
    for outer in 0..puzzle.len_outer() {
        result = result.max(count_energized(puzzle.clone(), Light {
            outer,
            inner: 0,
            direction: Direction::Right,
        }));
    }
    // Right column
    for outer in 0..puzzle.len_outer() {
        result = result.max(count_energized(puzzle.clone(), Light {
            outer,
            inner: puzzle.len_inner() - 1,
            direction: Direction::Left,
        }));
    }

    println!("{}", result);
}
