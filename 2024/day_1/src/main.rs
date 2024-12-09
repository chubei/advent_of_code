use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (mut first, mut second) = INPUT
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let first = words.next().unwrap().parse::<i32>().unwrap();
            let second = words.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    first.sort_unstable();
    second.sort_unstable();
    let result = first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    println!("{}", result);

    let mut first = vec![];
    let mut second = HashMap::new();
    for line in INPUT.lines() {
        let mut words = line.split_whitespace();
        let n1 = words.next().unwrap().parse::<i32>().unwrap();
        let n2 = words.next().unwrap().parse::<i32>().unwrap();
        first.push(n1);
        *second.entry(n2).or_insert(0) += 1;
    }
    let mut result = 0;
    for first in first {
        result += first * second.get(&first).unwrap_or(&0);
    }
    println!("{}", result);
}
