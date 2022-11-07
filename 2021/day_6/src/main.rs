const INPUT: [u8; 300] = [
    1, 1, 1, 2, 1, 5, 1, 1, 2, 1, 4, 1, 4, 1, 1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 4, 1, 1, 5, 1, 3, 1, 2,
    1, 1, 1, 2, 1, 1, 1, 4, 1, 1, 3, 1, 5, 1, 1, 1, 1, 3, 5, 5, 2, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1,
    1, 5, 4, 1, 1, 1, 1, 1, 3, 1, 1, 2, 4, 4, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 5, 1, 3, 1, 5, 1, 2,
    1, 1, 5, 1, 1, 1, 5, 3, 3, 1, 4, 1, 3, 1, 3, 1, 1, 1, 1, 3, 1, 4, 1, 1, 1, 1, 1, 2, 1, 1, 1, 4,
    2, 1, 1, 5, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 5, 1, 1, 1, 1, 3, 1, 1, 1, 1,
    1, 3, 4, 1, 2, 1, 3, 2, 1, 1, 2, 1, 1, 1, 1, 4, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1, 2, 1, 1, 4, 1, 1,
    1, 5, 3, 2, 2, 1, 1, 3, 1, 5, 1, 5, 1, 1, 1, 1, 1, 5, 1, 4, 1, 2, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1,
    1, 1, 1, 2, 1, 1, 1, 3, 1, 4, 3, 1, 4, 1, 3, 2, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    2, 1, 5, 1, 1, 1, 1, 2, 1, 1, 1, 3, 5, 1, 1, 1, 1, 5, 1, 1, 2, 1, 2, 4, 2, 2, 1, 1, 1, 5, 2, 1,
    1, 5, 1, 1, 1, 1, 5, 1, 1, 1, 2, 1,
];

fn num_fish_after(initial_state: u32, num_days: u32, memory: &mut [Option<usize>; 257]) -> usize {
    if num_days <= initial_state {
        1
    } else {
        num_fish_from_state_0_after(num_days - initial_state, memory)
    }
}

fn num_fish_from_state_0_after(num_days: u32, memory: &mut [Option<usize>; 257]) -> usize {
    if let Some(result) = memory[num_days as usize] {
        return result;
    }

    let mut result = 1;
    let mut creation_day = 1;
    while creation_day <= num_days {
        result += num_fish_after(8, num_days - creation_day, memory);
        creation_day += 7;
    }
    memory[num_days as usize] = Some(result);
    result
}

fn main() {
    let mut memory = [None; 257];
    println!(
        "{}",
        INPUT
            .iter()
            .map(|state| num_fish_after(*state as u32, 256, &mut memory))
            .sum::<usize>()
    );
}
