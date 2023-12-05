static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut result = 0;
    for line in INPUT.lines() {
        let first_digit = find_first_digit(line.as_bytes());
        let last_digit = find_last_digit(line.as_bytes());
        result += first_digit * 10 + last_digit;
    }
    println!("{}", result);
}

const STR_TO_DIGIT: [(&[u8], u32); 19] = [
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
    (b"0", 0),
    (b"1", 1),
    (b"2", 2),
    (b"3", 3),
    (b"4", 4),
    (b"5", 5),
    (b"6", 6),
    (b"7", 7),
    (b"8", 8),
    (b"9", 9),
];

fn find_first_digit(line: &[u8]) -> u32 {
    for index in 0..line.len() {
        for (digit, value) in STR_TO_DIGIT.iter() {
            if line[index..].starts_with(digit) {
                return *value;
            }
        }
    }
    unreachable!()
}

fn find_last_digit(line: &[u8]) -> u32 {
    for index in (0..line.len()).rev() {
        for (digit, value) in STR_TO_DIGIT.iter() {
            if line[index..].starts_with(digit) {
                return *value;
            }
        }
    }
    unreachable!()
}
