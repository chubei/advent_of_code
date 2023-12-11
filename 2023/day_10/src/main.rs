#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    OuterLess,
    OuterMore,
    InnerLess,
    InnerMore,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::OuterLess => Direction::OuterMore,
            Direction::OuterMore => Direction::OuterLess,
            Direction::InnerLess => Direction::InnerMore,
            Direction::InnerMore => Direction::InnerLess,
        }
    }

    fn is_vertical(&self) -> bool {
        match self {
            Direction::OuterLess | Direction::OuterMore => true,
            Direction::InnerLess | Direction::InnerMore => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeKind {
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl PipeKind {
    fn in_directions(&self) -> [Direction; 2] {
        match self {
            PipeKind::Vertical => [Direction::OuterLess, Direction::OuterMore],
            PipeKind::Horizontal => [Direction::InnerLess, Direction::InnerMore],
            PipeKind::TopLeft => [Direction::OuterLess, Direction::InnerLess],
            PipeKind::TopRight => [Direction::OuterLess, Direction::InnerMore],
            PipeKind::BottomLeft => [Direction::OuterMore, Direction::InnerLess],
            PipeKind::BottomRight => [Direction::OuterMore, Direction::InnerMore],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Trace {
    in_direction: Direction,
    out_direction: Direction,
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    kind: PipeKind,
    trace: Option<Trace>,
}

struct Map {
    pipes: Vec<Vec<Option<Pipe>>>,
}

impl Map {
    fn parse(input: &str) -> (Map, Position, Position) {
        let mut pipes = Vec::new();
        let mut starting_position = None;
        for (outer, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (inner, c) in line.chars().enumerate() {
                let pipe = match c {
                    '|' => Some(Pipe {
                        kind: PipeKind::Vertical,
                        trace: None,
                    }),
                    '-' => Some(Pipe {
                        kind: PipeKind::Horizontal,
                        trace: None,
                    }),
                    'L' => Some(Pipe {
                        kind: PipeKind::BottomLeft,
                        trace: None,
                    }),
                    'J' => Some(Pipe {
                        kind: PipeKind::BottomRight,
                        trace: None,
                    }),
                    '7' => Some(Pipe {
                        kind: PipeKind::TopRight,
                        trace: None,
                    }),
                    'F' => Some(Pipe {
                        kind: PipeKind::TopLeft,
                        trace: None,
                    }),
                    '.' => None,
                    'S' => {
                        starting_position = Some((outer, inner));
                        Some(Pipe {
                            // decide this later
                            kind: PipeKind::Vertical,
                            trace: None,
                        })
                    }
                    other => unreachable!("{}", other),
                };
                row.push(pipe);
            }
            pipes.push(row);
        }

        let mut map = Map { pipes };

        let starting_position = starting_position.unwrap();

        #[derive(Debug, Default)]
        struct Info {
            in_directions: Vec<Direction>,
            trace_in_direction: Option<Direction>,
            trace_out_direction: Option<Direction>,
            next_position: Option<Position>,
        }

        impl Info {
            fn add_out_connection(&mut self, out_direction: Direction, position: Position) {
                self.in_directions.push(out_direction.opposite());
                if self.next_position.is_none() {
                    // First time
                    self.next_position = Some(position);
                    self.trace_out_direction = Some(out_direction);
                } else {
                    // Second time
                    self.trace_in_direction = Some(out_direction.opposite());
                }
            }

            fn try_add_out_connection(&mut self, current_position: Position, position: Position, map: &Map) {
                if let Some((out_direction, _)) = directions(current_position, position, map) {
                    self.add_out_connection(out_direction, position);
                }
            }
        }

        let mut info = Info::default();
        if starting_position.0 > 0 {
            info.try_add_out_connection(starting_position, (starting_position.0 - 1, starting_position.1), &map);
        }
        if starting_position.0 < map.pipes.len() - 1 {
            info.try_add_out_connection(starting_position, (starting_position.0 + 1, starting_position.1), &map);
        }
        if starting_position.1 > 0 {
            info.try_add_out_connection(starting_position, (starting_position.0, starting_position.1 - 1), &map);
        }
        if starting_position.1 < map.pipes[0].len() - 1 {
            info.try_add_out_connection(starting_position, (starting_position.0, starting_position.1 + 1), &map);
        }
        assert_eq!(info.in_directions.len(), 2);
        info.in_directions.sort();
        let kind = match info.in_directions.as_slice() {
            [Direction::OuterLess, Direction::OuterMore] => PipeKind::Vertical,
            [Direction::InnerLess, Direction::InnerMore] => PipeKind::Horizontal,
            [Direction::OuterLess, Direction::InnerLess] => PipeKind::TopLeft,
            [Direction::OuterLess, Direction::InnerMore] => PipeKind::TopRight,
            [Direction::OuterMore, Direction::InnerLess] => PipeKind::BottomLeft,
            [Direction::OuterMore, Direction::InnerMore] => PipeKind::BottomRight,
            _ => unreachable!(),
        };
        let starting_pipe = map.pipes[starting_position.0][starting_position.1]
            .as_mut()
            .unwrap();
        starting_pipe.kind = kind;

        let trace = Trace {
            in_direction: info.trace_in_direction.unwrap(),
            out_direction: info.trace_out_direction.unwrap(),
        };
        starting_pipe.trace = Some(trace);

        (map, starting_position, info.next_position.unwrap())
    }
}

type Position = (usize, usize);

fn step(from: Position, through: Position, map: &Map) -> (Direction, Direction, Position) {
    let (direction, next_direction) = directions(from, through, map).unwrap();
    let next = match next_direction {
        Direction::OuterLess => (through.0 - 1, through.1),
        Direction::OuterMore => (through.0 + 1, through.1),
        Direction::InnerLess => (through.0, through.1 - 1),
        Direction::InnerMore => (through.0, through.1 + 1),
    };
    (direction, next_direction, next)
}

fn directions(
    from: Position,
    through: Position,
    map: &Map,
) -> Option<(Direction, Direction)> {
    let (from_outer, from_inner) = from;
    let (to_outer, to_inner) = through;

    let direction = if from_outer == to_outer {
        if from_inner + 1 == to_inner {
            Direction::InnerMore
        } else if to_inner + 1 == from_inner {
            Direction::InnerLess
        } else {
            return None;
        }
    } else if from_inner == to_inner {
        if from_outer + 1 == to_outer {
            Direction::OuterMore
        } else if to_outer + 1 == from_outer {
            Direction::OuterLess
        } else {
            return None;
        }
    } else {
        return None;
    };

    let Some(pipe) = map.pipes[through.0][through.1] else {
        return None;
    };
    let in_directions = pipe.kind.in_directions();
    if direction == in_directions[0] {
        Some((in_directions[0], in_directions[1].opposite()))
    } else if direction == in_directions[1] {
        Some((in_directions[1], in_directions[0].opposite()))
    } else {
        None
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (mut map, starting_position, mut current) = Map::parse(INPUT);
    let mut previous = starting_position;
    let mut num_steps = 1;
    loop {
        num_steps += 1;

        let (in_direction, out_direction, next) = step(previous, current, &map);
        map.pipes[current.0][current.1].as_mut().unwrap().trace = Some(Trace {
            in_direction,
            out_direction,
        });

        if next == starting_position {
            break;
        }
        previous = current;
        current = next;
    }
    println!("{}", num_steps / 2);

    // Find which direction marks enter of inner region.
    let enter_inner_direction = find_enter_inner_direction(&map);

    // Count
    let mut result = 0;
    for row in &map.pipes {
        let mut inner = false;
        for pipe in row {
            if inner {
                if !is_in_loop(pipe.as_ref()) {
                    result += 1;
                }
            }
            if let Some(direction) = vertical_trace_direction(pipe.as_ref()) {
                inner = direction == enter_inner_direction;
            }
        }
    }
    println!("{}", result);
}

fn is_in_loop(pipe: Option<&Pipe>) -> bool {
    pipe.and_then(|pipe| pipe.trace).is_some()
}

fn vertical_trace_direction(pipe: Option<&Pipe>) -> Option<Direction> {
    let trace = pipe?.trace?;
    if trace.in_direction.is_vertical() {
        Some(trace.in_direction)
    } else if trace.out_direction.is_vertical() {
        Some(trace.out_direction)
    } else {
        None
    }
}

fn find_enter_inner_direction(map: &Map) -> Direction {
    for row in &map.pipes {
        for pipe in row {
            if let Some(direction) = vertical_trace_direction(pipe.as_ref()) {
                return direction;
            }
        }
    }
    unreachable!()
}
