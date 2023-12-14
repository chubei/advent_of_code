use std::collections::{HashMap, hash_map::Entry};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Occupance {
    Empty,
    Round,
    Cube,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Puzzle {
    locations: Vec<Vec<Occupance>>,
}

impl Puzzle {
    fn num_rows(&self) -> usize {
        self.locations.len()
    }

    fn num_cols(&self) -> usize {
        self.locations[0].len()
    }

    fn col_north(&mut self, col: usize) {
        let mut next_position = 0;
        for row in 0..self.num_rows() {
            match self.locations[row][col] {
                Occupance::Round => {
                    if next_position != row {
                        self.locations[next_position][col] = Occupance::Round;
                        self.locations[row][col] = Occupance::Empty;
                    }
                    next_position += 1;
                }
                Occupance::Cube => {
                    next_position = row + 1;
                }
                Occupance::Empty => {}
            }
        }
    }

    fn north(&mut self) {
        for col in 0..self.num_cols() {
            self.col_north(col);
        }
    }

    fn col_south(&mut self, col: usize) {
        let mut next_position = self.num_rows() - 1;
        for row in (0..self.num_rows()).rev() {
            match self.locations[row][col] {
                Occupance::Round => {
                    if next_position != row {
                        self.locations[next_position][col] = Occupance::Round;
                        self.locations[row][col] = Occupance::Empty;
                    }
                    next_position -= 1;
                }
                Occupance::Cube => {
                    if row > 0 {
                        next_position = row - 1;
                    }
                }
                Occupance::Empty => {}
            }
        }
    }

    fn south(&mut self) {
        for col in 0..self.num_cols() {
            self.col_south(col);
        }
    }

    fn row_west(&mut self, row: usize) {
        let mut next_position = 0;
        for col in 0..self.num_cols() {
            match self.locations[row][col] {
                Occupance::Round => {
                    if next_position != col {
                        self.locations[row][next_position] = Occupance::Round;
                        self.locations[row][col] = Occupance::Empty;
                    }
                    next_position += 1;
                }
                Occupance::Cube => {
                    next_position = col + 1;
                }
                Occupance::Empty => {}
            }
        }
    }

    fn west(&mut self) {
        for row in 0..self.num_rows() {
            self.row_west(row);
        }
    }

    fn row_east(&mut self, row: usize) {
        let mut next_position = self.num_cols() - 1;
        for col in (0..self.num_cols()).rev() {
            match self.locations[row][col] {
                Occupance::Round => {
                    if next_position != col {
                        self.locations[row][next_position] = Occupance::Round;
                        self.locations[row][col] = Occupance::Empty;
                    }
                    next_position -= 1;
                }
                Occupance::Cube => {
                    if col > 0 {
                        next_position = col - 1;
                    }
                }
                Occupance::Empty => {}
            }
        }
    }

    fn east(&mut self) {
        for row in 0..self.num_rows() {
            self.row_east(row);
        }
    }

    fn col_north_load(&self, col: usize) -> usize {
        let mut result = 0;
        for row in 0..self.num_rows() {
            if let Occupance::Round = self.locations[row][col] {
                result += self.num_rows() - row;
            }
        }
        result
    }

    fn north_load(&self) -> usize {
        let mut result = 0;
        for col in 0..self.num_cols() {
            result += self.col_north_load(col);
        }
        result
    }

    fn parse(input: &str) -> Puzzle {
        let mut locations = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    'O' => row.push(Occupance::Round),
                    '.' => row.push(Occupance::Empty),
                    '#' => row.push(Occupance::Cube),
                    _ => panic!("Invalid character"),
                }
            }
            locations.push(row);
        }
        Puzzle { locations }
    }
}

fn main() {
    let mut puzzle = Puzzle::parse(INPUT);
    let mut history = HashMap::new();
    history.insert(puzzle.clone(), 0);
    let mut loop_end = 0;
    let loop_start = loop {
        puzzle.north();
        puzzle.west();
        puzzle.south();
        puzzle.east();
        loop_end += 1;
        match history.entry(puzzle.clone()) {
            Entry::Occupied(entry) => {
                break *entry.get();
            }
            Entry::Vacant(entry) => {
                entry.insert(loop_end);
            }
        }
    };
    let index = (1_000_000_000 - loop_start) % (loop_end - loop_start) + loop_start;
    let puzzle = history
        .iter()
        .find(|(_, &value)| value == index)
        .unwrap()
        .0;
    println!("{}", puzzle.north_load());
}
