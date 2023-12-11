const INPUT: &str = include_str!("input.txt");

fn extrapolate(sequence: Vec<i32>) -> i32 {
    let mut diffs = vec![sequence];
    loop {
        let seq = diffs.last().unwrap();
        let diff = seq.iter().skip(1).zip(seq.iter()).map(|(a, b)| a - b).collect::<Vec<_>>();
        if diff.iter().all(|d| *d == 0) {
            return diffs.iter().enumerate().map(|(index, seq)| seq[0] * if index % 2 == 0 { 1 } else { -1 }).sum::<i32>();
        }
        diffs.push(diff);
    }
}

fn main() {
    let mut result = 0;
    for line in INPUT.lines() {
        let sequence = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        result += extrapolate(sequence);
    }
    println!("{}", result);
}
