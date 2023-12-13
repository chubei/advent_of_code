use std::convert::identity;

struct Universe {
    galaxies: Vec<Vec<bool>>,
    outer_expansions: Vec<usize>,
    inner_expansions: Vec<usize>,
}

impl Universe {
    fn parse(input: &str) -> Self {
        let galaxies: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        let outer_expansions = galaxies.iter().enumerate().filter_map(|(index, row)| (!row.iter().copied().any(identity)).then(|| index)).collect();
        let inner_expansions = (0..galaxies[0].len()).filter(|index| !galaxies.iter().any(|row| row[*index])).collect();
        Self { galaxies, outer_expansions, inner_expansions }
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let universe = Universe::parse(INPUT);

    let mut positions = vec![];
    for (outer, row) in universe.galaxies.iter().enumerate() {
        for (inner, &cell) in row.iter().enumerate() {
            if cell {
                positions.push((outer, inner));
            }
        }
    }

    let mut result = 0;
    for (index, position) in positions.iter().enumerate() {
        for other in &positions[index + 1..] {
            result += distance(position.0, other.0, &universe.outer_expansions) + distance(position.1, other.1, &universe.inner_expansions);
        }
    }
    println!("Result: {}", result);
}

fn distance(a: usize, b: usize, expansions: &[usize]) -> usize {
    if a < b {
        distance(b, a, expansions)
    } else {
        let expansion = expansions.iter().filter(|&&index| index < a && index > b).count();
        a - b + expansion * 999_999
    }
}
