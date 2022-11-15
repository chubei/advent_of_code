const INPUT: &[u8] = b"8261344656
7773351175
7527856852
1763614673
8674556743
6853382153
4135852388
2846715522
7477425863
4723888888";

fn main() {
    let mut input = INPUT
        .split(|&b| b == b'\n')
        .map(|line| line.iter().map(|&b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    let num_octopuses = input.iter().flatten().count() as u32;
    loop {
        count += 1;
        if step(&mut input) == num_octopuses {
            println!("{}", count);
            break;
        }
    }
}

fn step(energies: &mut Vec<Vec<u8>>) -> u32 {
    energies.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|energy| {
            *energy += 1;
        });
    });

    let mut result = 0;
    let mut flag = vec![vec![false; energies[0].len()]; energies.len()];
    while flash_one(energies, &mut flag) {
        result += 1;
    }

    for i in 0..energies.len() {
        for j in 0..energies[i].len() {
            if flag[i][j] {
                energies[i][j] = 0;
            }
        }
    }

    result
}

fn flash_one(energies: &mut Vec<Vec<u8>>, flag: &mut [Vec<bool>]) -> bool {
    for i in 0..energies.len() {
        for j in 0..energies[i].len() {
            if energies[i][j] > 9 && !flag[i][j] {
                flag[i][j] = true;
                if i > 0 {
                    energies[i - 1][j] += 1;
                }
                if i < energies.len() - 1 {
                    energies[i + 1][j] += 1;
                }
                if j > 0 {
                    energies[i][j - 1] += 1;
                }
                if j < energies[i].len() - 1 {
                    energies[i][j + 1] += 1;
                }
                if i > 0 && j > 0 {
                    energies[i - 1][j - 1] += 1;
                }
                if i > 0 && j < energies[i].len() - 1 {
                    energies[i - 1][j + 1] += 1;
                }
                if i < energies.len() - 1 && j > 0 {
                    energies[i + 1][j - 1] += 1;
                }
                if i < energies.len() - 1 && j < energies[i].len() - 1 {
                    energies[i + 1][j + 1] += 1;
                }
                return true;
            }
        }
    }
    false
}
