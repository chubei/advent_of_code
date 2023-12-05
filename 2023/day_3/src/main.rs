struct Matrix {
    data: Vec<Vec<u8>>
}

impl Matrix {
    fn new(outer: usize, inner: usize, fill: u8) -> Self {
        Self {
            data: vec![vec![fill; inner]; outer]
        }
    }

    fn outer_len(&self) -> usize {
        self.data.len()
    }

    fn inner_len(&self) -> usize {
        self.data[0].len()
    }

    fn new_like(other: &Self, fill: u8) -> Self {
        Self::new(other.outer_len(), other.inner_len(), fill)
    }

    fn parse(input: &str) -> Self {
        let mut data = Vec::<Vec<u8>>::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.as_bytes() {
                row.push(*c);
            }
            assert!(data.is_empty() || data[0].len() == row.len());
            data.push(row);
        }
        Self { data }
    }

    fn find_all_numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();
        for (outer_index, line) in self.data.iter().enumerate() {
            let mut inner_index = 0;
            while inner_index < line.len() {
                let char = line[inner_index];
                if char.is_ascii_digit() {
                    let mut number = (char - b'0') as u32;
                    let mut i = inner_index + 1;
                    while i < line.len() {
                        let char = line[i];
                        if !char.is_ascii_digit() {
                            break;
                        }
                        number = number * 10 + (char - b'0') as u32;
                        i += 1;
                    }
                    numbers.push(Number {
                        outer_index,
                        inner_range: inner_index..i,
                        value: number
                    });
                    inner_index = i + 1;
                } else {
                    inner_index += 1;
                }
            }
        }
        numbers
    }
}

struct Number {
    outer_index: usize,
    inner_range: std::ops::Range<usize>,
    value: u32
}

impl Number {
    fn is_inner_adjacent(&self, index: usize) -> bool {
        (self.inner_range.start..self.inner_range.end + 1).contains(&index) || (self.inner_range.start > 0 &&  self.inner_range.start - 1 == index)
    }

    fn is_adjacent(&self, outer_index: usize, inner_index: usize) -> bool {
        self.is_inner_adjacent(inner_index) && is_adjacent(self.outer_index, outer_index)
    }
}

fn is_adjacent(num1: usize, num2: usize) -> bool {
    if num1 > num2 {
        num1 - num2 == 1
    } else {
        num2 - num1 <= 1
    }
}

const INPUT: &str = include_str!("input.txt");

fn is_symbol(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

fn main() {
    let matrix = Matrix::parse(INPUT);
    let numbers = matrix.find_all_numbers();
    let mut result = 0;
    for outer_index in 0..matrix.outer_len() {
        for inner_index in 0..matrix.inner_len() {
            if matrix.data[outer_index][inner_index] == b'*' {
                let mut adjacent_numbers = Vec::new();
                for number in &numbers {
                    if number.is_adjacent(outer_index, inner_index) {
                        adjacent_numbers.push(number);
                    }
                }
                if adjacent_numbers.len() == 2 {
                    result += adjacent_numbers[0].value * adjacent_numbers[1].value;
                }
            }
        }
    }
    println!("{}", result);
}
