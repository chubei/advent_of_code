use regex::Regex;

fn main() {
    let regex = Regex::new(r"don't|do|mul\((\d+),(\d+)\)").unwrap();
    let mut state = (0, true);
    regex
        .captures_iter(include_str!("input.txt"))
        .for_each(|cap| {
            if &cap[0] == "don't" {
                state.1 = false;
            } else if &cap[0] == "do" {
                state.1 = true;
            } else if state.1 {
                let a = cap[1].parse::<i32>().unwrap();
                let b = cap[2].parse::<i32>().unwrap();
                state.0 += a * b;
            }
        });
    println!("{}", state.0);
}
