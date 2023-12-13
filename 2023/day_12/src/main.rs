#[derive(Debug)]
struct Puzzle {
    damaged: Vec<Option<bool>>,
    segments: Vec<usize>,
}

fn solve(puzzle: &Puzzle) -> usize {
    let num_damaged: usize = puzzle.segments.iter().copied().sum();
    // Outer: index of `damaged`.
    // Middle: 0 -> false, 1 -> true.
    // Inner: number of damaged.
    let mut dp = vec![[vec![0; num_damaged + 1], vec![0; num_damaged + 1],]; puzzle.damaged.len()];

    match puzzle.damaged[0] {
        Some(true) => dp[0][1][1] = 1,
        Some(false) => dp[0][0][0] = 1,
        None => {
            dp[0][0][0] = 1;
            dp[0][1][1] = 1;
        }
    }

    for (index, &current) in puzzle.damaged.iter().enumerate().skip(1) {
        for num_damaged in 0..=num_damaged {
            for last_is_damaged in [false, true] {
                let Some(expected) = find_expected(num_damaged, last_is_damaged, &puzzle.segments) else {
                    continue;
                };
                let last_is_damaged = if last_is_damaged { 1 } else { 0 };
                match (current, expected) {
                    (Some(true), None | Some(true)) | (None, Some(true)) => {
                        dp[index][1][num_damaged + 1] += dp[index - 1][last_is_damaged][num_damaged];
                    }
                    (Some(false), None | Some(false)) | (None, Some(false)) => {
                        dp[index][0][num_damaged] += dp[index - 1][last_is_damaged][num_damaged];
                    }
                    (None, None) => {
                        dp[index][0][num_damaged] += dp[index - 1][last_is_damaged][num_damaged];
                        dp[index][1][num_damaged + 1] += dp[index - 1][last_is_damaged][num_damaged];
                    }
                    (Some(true), Some(false)) | (Some(false), Some(true)) => {
                        continue;
                    }
                }
            }
        }
    }

    dp[puzzle.damaged.len() - 1][0][num_damaged] + dp[puzzle.damaged.len() - 1][1][num_damaged]
}

/// - `None`: Impossible
/// - `Some(None)`: Arbitrary
/// - `Some(Some)`: Fixed
fn find_expected(mut num_damaged: usize, last_is_damaged: bool, segments: &[usize]) -> Option<Option<bool>> {
    let mut index = 0;
    while index < segments.len() && num_damaged >= segments[index] {
        num_damaged -= segments[index];
        index += 1;
    }

    if index == segments.len() {
        if num_damaged == 0 {
            Some(Some(false))
        } else {
            None
        }
    } else {
        if num_damaged == 0 {
            if last_is_damaged {
                Some(Some(false))
            } else {
                Some(None)
            }
        } else {
            if last_is_damaged {
                Some(Some(true))
            } else {
                None
            }
        }
    }
}

fn parse(input: &str) -> Puzzle {
    let mut parts = input.split_whitespace();
    let damaged = parts
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            '#' => Some(true),
            '.' => Some(false),
            '?' => None,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let damaged = vec![damaged; 5].join(&None);
    let segments = parts
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    let segments = vec![segments; 5].into_iter().flatten().collect();
    Puzzle { damaged, segments }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let lines = INPUT.lines().collect::<Vec<_>>();
    println!(
        "{}",
        lines
            .iter()
            .map(|line| solve(&parse(line)))
            .reduce(std::ops::Add::add)
            .unwrap()
    );
}
