use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "u"),
            Direction::Down => write!(f, "d"),
            Direction::Left => write!(f, "l"),
            Direction::Right => write!(f, "r"),
        }
    }
}

struct Puzzle {
    heat_losses: Vec<Vec<u32>>,
}

impl Puzzle {
    fn width(&self) -> usize {
        self.heat_losses[0].len()
    }

    fn height(&self) -> usize {
        self.heat_losses.len()
    }
}

type Position = (usize, usize);

fn solve(puzzle: &Puzzle) -> u32 {
    // outer, inner, direction, num steps
    let mut dp =
        vec![vec![[[None; MAX_STEPS_BEFORE_TURN_PLUS_1]; 4]; puzzle.width()]; puzzle.height()];
    dp[0][1][Direction::Right as usize][1] = Some(puzzle.heat_losses[0][1]);
    dp[1][0][Direction::Down as usize][1] = Some(puzzle.heat_losses[1][0]);

    loop {
        let mut changed = false;
        changed |= top_down_scan(puzzle, &mut dp);
        changed |= bottom_up_scan(puzzle, &mut dp);
        if !changed {
            break;
        }
    }

    *dp[puzzle.height() - 1][puzzle.width() - 1]
        .iter()
        .map(|step_dps| &step_dps[MIN_STEPS_BEFORE_TURN_OR_STOP..])
        .flatten()
        .flatten()
        .min()
        .unwrap()
}

fn top_down_scan(
    puzzle: &Puzzle,
    dp: &mut Vec<Vec<[[Option<u32>; MAX_STEPS_BEFORE_TURN_PLUS_1]; 4]>>,
) -> bool {
    let mut result = false;
    for outer in 0..puzzle.height() {
        for inner in 0..puzzle.width() {
            let position = (outer, inner);
            if outer > 0 {
                result |= state_transfer(puzzle, dp, position, (outer - 1, inner), Direction::Down);
            }
            if inner > 0 {
                result |=
                    state_transfer(puzzle, dp, position, (outer, inner - 1), Direction::Right);
            }
        }
    }
    result
}

fn bottom_up_scan(
    puzzle: &Puzzle,
    dp: &mut Vec<Vec<[[Option<u32>; MAX_STEPS_BEFORE_TURN_PLUS_1]; 4]>>,
) -> bool {
    let mut result = false;
    for outer in (0..puzzle.height()).rev() {
        for inner in (0..puzzle.width()).rev() {
            let position = (outer, inner);
            if outer + 1 < puzzle.height() {
                result |= state_transfer(puzzle, dp, position, (outer + 1, inner), Direction::Up);
            }
            if inner + 1 < puzzle.width() {
                result |= state_transfer(puzzle, dp, position, (outer, inner + 1), Direction::Left);
            }
        }
    }
    result
}

const MIN_STEPS_BEFORE_TURN_OR_STOP: usize = 4;
const MAX_STEPS_BEFORE_TURN_PLUS_1: usize = 11;

fn state_transfer_step_1(
    puzzle: &Puzzle,
    dp: &mut Vec<Vec<[[Option<u32>; MAX_STEPS_BEFORE_TURN_PLUS_1]; 4]>>,
    position: Position,
    from_position: Position,
    direction: Direction,
) -> bool {
    let mut last_steps = vec![];
    for from_direction in Direction::all() {
        if from_direction == direction || from_direction == direction.opposite() {
            continue;
        }
        for num_steps in MIN_STEPS_BEFORE_TURN_OR_STOP..MAX_STEPS_BEFORE_TURN_PLUS_1 {
            last_steps.push(
                dp[from_position.0][from_position.1][from_direction as usize][num_steps]
                    .map(|heat_loss| heat_loss + puzzle.heat_losses[position.0][position.1]),
            );
        }
    }
    let dp = &mut dp[position.0][position.1][direction as usize][1];
    let new = last_steps.into_iter().flatten().chain(*dp).min();
    let result = *dp != new;
    *dp = new;
    result
}

fn state_transfer_step_n(
    puzzle: &Puzzle,
    dp: &mut Vec<Vec<[[Option<u32>; MAX_STEPS_BEFORE_TURN_PLUS_1]; 4]>>,
    position: Position,
    from_position: Position,
    direction: Direction,
    num_steps: usize,
) -> bool {
    if let Some(heat_loss) = dp[from_position.0][from_position.1][direction as usize][num_steps - 1]
    {
        let dp = &mut dp[position.0][position.1][direction as usize][num_steps];
        let new = Some(
            dp.unwrap_or(u32::MAX)
                .min(heat_loss + puzzle.heat_losses[position.0][position.1]),
        );
        let result = *dp != new;
        *dp = new;
        result
    } else {
        false
    }
}

fn state_transfer(
    puzzle: &Puzzle,
    dp: &mut Vec<Vec<[[Option<u32>; MAX_STEPS_BEFORE_TURN_PLUS_1]; 4]>>,
    position: Position,
    from_position: Position,
    direction: Direction,
) -> bool {
    let mut result = false;
    result |= state_transfer_step_1(puzzle, dp, position, from_position, direction);
    for num_steps in 2..MAX_STEPS_BEFORE_TURN_PLUS_1 {
        result |= state_transfer_step_n(puzzle, dp, position, from_position, direction, num_steps);
    }
    result
}

const INPUT: &str = include_str!("input.txt");

fn print_dp(puzzle: &Puzzle, dp: &Vec<Vec<[[Option<u32>; MAX_STEPS_BEFORE_TURN_PLUS_1]; 4]>>) {
    for (outer, row) in dp.iter().enumerate() {
        for (inner, cell) in row.iter().enumerate() {
            let mut best: Option<(u32, Direction, usize)> = None;
            for direction in Direction::all() {
                for num_steps in 1..4 {
                    let heat_loss = cell[direction as usize][num_steps];
                    let new_best = heat_loss.into_iter().chain(best.map(|best| best.0)).min();
                    if new_best != best.map(|best| best.0) {
                        best = new_best.map(|best| (best, direction, num_steps));
                    }
                }
            }
            let min = best
                .map(|best| format!("{:3}", best.0))
                .unwrap_or("  x".to_string());
            let direction = best
                .map(|best| best.1.to_string())
                .unwrap_or("x".to_string());
            let num_steps = best
                .map(|best| best.2.to_string())
                .unwrap_or("x".to_string());

            print!(
                "{}{}{}/{} ",
                min, direction, num_steps, puzzle.heat_losses[outer][inner]
            );
        }
        println!();
    }
    println!();
}

fn main() {
    let heat_losses = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let puzzle = Puzzle { heat_losses };
    println!("{}", solve(&puzzle));
}
