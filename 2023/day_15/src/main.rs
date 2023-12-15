const INPUT: &str = include_str!("input.txt");

struct Instruction {
    label: &'static [u8],
    kind: InstructionKind,
}

enum InstructionKind {
    Set(u32),
    Clear,
}

impl Instruction {
    fn parse(input: &'static str) -> Self {
        if input.contains('-') {
            let mut parts = input.split('-');
            let label = parts.next().unwrap().as_bytes();
            Self {
                label,
                kind: InstructionKind::Clear,
            }
        } else {
            let mut parts = input.split('=');
            let label = parts.next().unwrap().as_bytes();
            let value = parts.next().unwrap().parse().unwrap();
            Self {
                label,
                kind: InstructionKind::Set(value),
            }
        }
    }
}

fn hash(input: &[u8]) -> usize {
    let mut result = 0;
    for &byte in input {
        result = (result + byte as usize) * 17 % 256;
    }
    result
}

type Box = Vec<(&'static [u8], u32)>;

fn run(bos: &mut Box, instruction: Instruction) {
    let index = bos.iter().enumerate().find(|(_, (label, _))| *label == instruction.label).map(|(index, _)| index);
    match instruction.kind {
        InstructionKind::Set(value) => {
            if let Some(index) = index {
                bos[index].1 = value;
            } else {
                bos.push((instruction.label, value));
            }
        }
        InstructionKind::Clear => {
            if let Some(index) = index {
                bos.remove(index);
            }
        }
    }
}

type Lenses = Vec<Box>;

fn main() {
    let mut lenses: Lenses = vec![Box::default(); 256];
    for instruction in INPUT.split(',') {
        let instruction = Instruction::parse(instruction);
        let box_index = hash(instruction.label);
        run(&mut lenses[box_index], instruction);
    }

    println!("{}", lenses.into_iter().enumerate().map(|(box_index, bos)| {
        bos.into_iter().enumerate().map(|(lens_index, (_, value))| (box_index + 1) * (lens_index + 1) * value as usize).sum::<usize>()
    }).sum::<usize>());
}
