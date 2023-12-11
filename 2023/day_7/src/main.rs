use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand([u8; 5]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const JOKER: u8 = 1;

impl Hand {
    fn typ(&self) -> Type {
        let mut counts = HashMap::new();
        let mut num_jokers = 0;
        for card in self.0 {
            if card == JOKER {
                num_jokers += 1;
            } else {
                *counts.entry(card).or_default() += 1;
            }
        }
        let mut counts = counts.values().copied().collect::<Vec<_>>();
        counts.sort();
        if let Some(max_non_joker) = counts.last_mut() {
            *max_non_joker += num_jokers;
        } else {
            counts = vec![5];
        }
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => Type::HighCard,
            [1, 1, 1, 2] => Type::OnePair,
            [1, 2, 2] => Type::TwoPair,
            [1, 1, 3] => Type::ThreeOfAKind,
            [2, 3] => Type::FullHouse,
            [1, 4] => Type::FourOfAKind,
            [5] => Type::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.typ().cmp(&other.typ()) {
            Ordering::Equal => self.0.cmp(&other.0),
            other => other,
        }
    }
}

// Parse something like "TQA26 14"
fn parse_bid(line: &str) -> (Hand, usize) {
    let mut bid = line.split_whitespace();
    let hand = bid.next().unwrap().chars().map(|c| match c {
        'T' => 10,
        'J' => JOKER,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as u8,
    }).collect::<Vec<_>>();
    let hand = Hand(hand.try_into().unwrap());
    let bid = bid.next().unwrap().parse().unwrap();
    (hand, bid)
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut bids = INPUT.lines().map(parse_bid).collect::<Vec<_>>();
    bids.sort_by_key(|(hand, _)| *hand);
    let result = bids.iter().enumerate().map(|(i, (_, bid))| (i + 1) * bid).sum::<usize>();
    println!("{}", result);
}
