const TEMPLATE: &[u8] = b"CVKKFSSNNHNPSPPKBHPB";

const RULES: &[u8] = b"OF -> S
VO -> F
BP -> S
FC -> S
PN -> K
HC -> P
PP -> N
FK -> V
KN -> C
BO -> O
KS -> B
FF -> S
KC -> B
FV -> C
VF -> N
HS -> H
OS -> F
VC -> S
VP -> P
BC -> O
HF -> F
HO -> F
PC -> B
CC -> K
NB -> N
KK -> N
KP -> V
BH -> H
BF -> O
OB -> F
VK -> P
FB -> O
NP -> B
CB -> C
PS -> S
KO -> V
SP -> C
BK -> O
NN -> O
OC -> F
VB -> B
ON -> K
NK -> B
CK -> H
NH -> N
CV -> C
PF -> P
PV -> V
CP -> N
FP -> N
SB -> B
SN -> N
KF -> F
HP -> S
BN -> V
NF -> B
PO -> O
CH -> O
VV -> S
OV -> V
SF -> P
BV -> S
FH -> V
CN -> H
VH -> V
HB -> B
FN -> P
OH -> S
SK -> H
OP -> H
VN -> V
HN -> P
BS -> S
CF -> B
PB -> H
SS -> K
NV -> P
FS -> N
CS -> O
OK -> B
CO -> O
VS -> F
OO -> B
NO -> H
SO -> F
HH -> K
FO -> H
SH -> O
HV -> B
SV -> N
PH -> F
BB -> P
KV -> B
KB -> H
KH -> N
NC -> P
SC -> S
PK -> B
NS -> V
HK -> B";

use std::collections::HashMap;

fn main() {
    let rules = RULES
        .split(|b| *b == b'\n')
        .map(|rule| ([rule[0], rule[1]], rule[rule.len() - 1]))
        .collect::<HashMap<_, _>>();

    let mut memory: HashMap<[u8; 3], [usize; 26]> = HashMap::new();

    let mut result = [0; 26];
    for pair in TEMPLATE.iter().zip(TEMPLATE.iter().skip(1)) {
        add(
            new_characters_from_pair_after_step([*pair.0, *pair.1], 40, &rules, &mut memory),
            &mut result,
        );
    }

    for c in TEMPLATE {
        result[(*c - b'A') as usize] += 1;
    }

    println!(
        "{}",
        result.iter().max().unwrap() - result.iter().filter(|v| **v > 0).min().unwrap()
    );
}

fn new_characters_from_pair_after_step(
    pair: [u8; 2],
    step: u8,
    rules: &HashMap<[u8; 2], u8>,
    memory: &mut HashMap<[u8; 3], [usize; 26]>,
) -> [usize; 26] {
    if step == 0 {
        return [0; 26];
    }

    match rules.get(&pair) {
        Some(&c) => {
            if let Some(&memory) = memory.get(&[pair[0], pair[1], step]) {
                memory
            } else {
                let mut result = [0; 26];
                result[(c - b'A') as usize] = 1;

                add(
                    new_characters_from_pair_after_step([pair[0], c], step - 1, rules, memory),
                    &mut result,
                );
                add(
                    new_characters_from_pair_after_step([c, pair[1]], step - 1, rules, memory),
                    &mut result,
                );

                memory.insert([pair[0], pair[1], step], result);
                result
            }
        }
        None => [0; 26],
    }
}

fn add(a: [usize; 26], b: &mut [usize; 26]) {
    for i in 0..26 {
        b[i] += a[i];
    }
}
