use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum Order {
    Ascending,
    Descending,
    Any,
}

fn is_safe(numbers: &[u32], order: Order) -> bool {
    if numbers.len() < 2 {
        return true;
    }
    match numbers[0].cmp(&numbers[1]) {
        Ordering::Less => {
            if order == Order::Descending {
                return false;
            }
            if numbers[1] - numbers[0] > 3 {
                return false;
            }
            is_safe(&numbers[1..], Order::Ascending)
        }
        Ordering::Greater => {
            if order == Order::Ascending {
                return false;
            }
            if numbers[0] - numbers[1] > 3 {
                return false;
            }
            is_safe(&numbers[1..], Order::Descending)
        }
        Ordering::Equal => false,
    }
}

fn main() {
    let result = include_str!("input.txt")
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            for i in 0..numbers.len() {
                let mut numbers = numbers.clone();
                numbers.remove(i);
                if is_safe(&numbers, Order::Any) {
                    return 1;
                }
            }
            0
        })
        .sum::<u32>();
    println!("{}", result);
}
