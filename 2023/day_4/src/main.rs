use std::collections::BTreeMap;

struct Card {
    id: u32,
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn winning_count(&self) -> u32 {
        let mut count = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                count += 1;
            }
        }
        count
    }

    fn point(&self) -> u32 {
        let count = self.winning_count();
        if count > 0 {
            2u32.pow(count - 1)
        } else {
            0
        }
    }

    /// Parse something like "Card   1: 95 57 30 62 11  5  9  3 72 87 | 94 72 74 98 23 57 62 14 30  3 73 49 80 96 20 60 17 35 11 63 87  9  6  5 95"
    fn parse(input: &str) -> Card {
        let mut parts = input.split(":");
        let id = parts.next().unwrap();
        let id = id.trim();
        let id = id.strip_prefix("Card").unwrap();
        let id = id.trim();
        let id = id.parse::<u32>().unwrap();
        let numbers = parts.next().unwrap();
        let mut sets = numbers.split("|");
        let winning_numbers = sets.next().unwrap();
        let numbers = sets.next().unwrap();
        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        let numbers = numbers
            .split_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        Card {
            id,
            winning_numbers,
            numbers,
        }
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut cards = BTreeMap::new();
    for line in INPUT.lines() {
        let card = Card::parse(line);
        cards.insert(card.id, (card, 1));
    }

    let card_ids = cards.keys().copied().collect::<Vec<_>>();
    for card_id in card_ids {
        let (card, count) = cards.get(&card_id).unwrap();
        let winning_count = card.winning_count();
        let count = *count;
        for i in 0..winning_count {
            cards.get_mut(&(card_id + i + 1)).unwrap().1 += count;
        }
    }
    let result = cards.values().map(|(_, count)| count).sum::<u32>();
    println!("{}", result);
}
