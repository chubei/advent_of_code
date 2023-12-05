struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

struct Bag {
    red: u32,
    blue: u32,
    green: u32,
}

fn round_is_possible(bag: &Bag, round: &Round) -> bool {
    bag.red >= round.red && bag.blue >= round.blue && bag.green >= round.green
}

fn game_is_possible(bag: &Bag, game: &Game) -> bool {
    for round in &game.rounds {
        if !round_is_possible(bag, round) {
            return false;
        }
    }
    true
}

fn fewest_ball_bag(game: &Game) -> Bag {
    let mut bag = Bag { red: 0, blue: 0, green: 0 };
    for round in &game.rounds {
        bag.red = bag.red.max(round.red);
        bag.blue = bag.blue.max(round.blue);
        bag.green = bag.green.max(round.green);
    }
    bag
}

fn bag_power(bag: &Bag) -> u32 {
    bag.red * bag.blue * bag.green
}

// Parse something like "Game 4: 4 blue, 8 green, 5 red; 6 red, 7 blue, 9 green; 2 green, 2 red, 2 blue; 2 green, 6 blue, 9 red; 10 red, 9 green"
fn parse_game(game: &str) -> Game {
    let mut game = game.split(": ");
    let id = game.next().unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();
    let mut rounds = Vec::new();
    for round in game.next().unwrap().split("; ") {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        let colors = round.split(", ");
        for color in colors {
            let mut color = color.split(" ");
            let count = color.next().unwrap().parse::<u32>().unwrap();
            match color.next().unwrap() {
                "red" => red = count,
                "blue" => blue = count,
                "green" => green = count,
                _ => panic!("Unknown color"),
            }
        }
        rounds.push(Round { red, blue, green });
    }
    Game { id, rounds }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut sum = 0;
    for game in INPUT.lines() {
        let game = parse_game(game);
        let bag = fewest_ball_bag(&game);
        sum += bag_power(&bag);
    }
    println!("{}", sum);
}
