struct Puzzle {
    pattern: Vec<Vec<bool>>,
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        let pattern = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect();
        Puzzle { pattern }
    }

    fn is_vertically_reflected(&self, position: usize) -> bool {
        let mut diff = 0;
        for left in 0..position {
            let right = 2 * position - left - 1;
            if right < self.num_cols() {
                for row in 0..self.num_rows() {
                    if self.pattern[row][left] != self.pattern[row][right] {
                        diff += 1;
                        if diff > 1 {
                            return false;
                        }
                    }
                }
            }
        }
        diff == 1
    }

    fn is_horizontally_reflected(&self, position: usize) -> bool {
        let mut diff = 0;
        for top in 0..position {
            let bottom = 2 * position - top - 1;
            if bottom < self.num_rows() {
                for col in 0..self.num_cols() {
                    if self.pattern[top][col] != self.pattern[bottom][col] {
                        diff += 1;
                        if diff > 1 {
                            return false;
                        }
                    }
                }
            }
        }
        diff == 1
    }

    fn num_rows(&self) -> usize {
        self.pattern.len()
    }

    fn num_cols(&self) -> usize {
        self.pattern[0].len()
    }
}

fn solve(puzzle: &Puzzle) -> usize {
    for index in 1..puzzle.num_rows() {
        if puzzle.is_horizontally_reflected(index) {
            return index * 100;
        }
    }
    for index in 1..puzzle.num_cols() {
        if puzzle.is_vertically_reflected(index) {
            return index;
        }
    }
    unreachable!()
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{}", INPUT.split("\n\n").map(|input| solve(&Puzzle::parse(input))).sum::<usize>());
}
