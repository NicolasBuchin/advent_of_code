use std::{cmp::Reverse, collections::BinaryHeap};

pub fn crossed_wires(input: &str) -> String {
    let instructions = parse(input);
    solve(&instructions)
}

#[derive(PartialEq, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Instruction {
    left: [u8; 3],
    right: [u8; 3],
    op: Op,
    destination: [u8; 3],
}

fn solve(instructions: &[Instruction]) -> String {
    let mut wrong_gates = BinaryHeap::new();
    for instruction in instructions {
        if (instruction.destination[0] == b'z'
            && instruction.op != Op::Xor
            && instruction.destination != [b'z', b'4', b'5'])
            || (instruction.destination[0] != b'z'
                && instruction.left[0] + instruction.right[0] != 0xF1
                && instruction.op == Op::Xor)
            || (instruction.op == Op::Xor
                && instruction.left[0] + instruction.right[0] == 0xF1
                && instruction.destination != [b'z', b'0', b'0']
                && !instructions.iter().any(|i| {
                    i.op == Op::Xor && (i.left == instruction.destination || i.right == instruction.destination)
                }))
            || (instruction.op == Op::And
                && instruction.left != [b'y', b'0', b'0']
                && !instructions.iter().any(|i| {
                    i.op == Op::Or && (i.left == instruction.destination || i.right == instruction.destination)
                }))
        {
            wrong_gates.push(Reverse(instruction.destination));
        }
    }
    let mut gates_bytes = Vec::new();
    while let Some(Reverse(bytes)) = wrong_gates.pop() {
        if !gates_bytes.is_empty() {
            gates_bytes.push(b',');
        }
        gates_bytes.extend_from_slice(&bytes);
    }
    String::from_utf8(gates_bytes).unwrap()
}

fn parse(input: &str) -> Vec<Instruction> {
    let bytes = input.as_bytes();
    let mut instructions = Vec::new();
    let mut i = 0;

    while bytes[i] != b'\n' {
        i += 7;
    }
    i += 1;

    while i < bytes.len() {
        let left = [bytes[i], bytes[i + 1], bytes[i + 2]];
        let op = match bytes[i + 4] {
            b'A' => {
                i += 8;
                Op::And
            }
            b'O' => {
                i += 7;
                Op::Or
            }
            b'X' => {
                i += 8;
                Op::Xor
            }
            _ => unreachable!(),
        };
        let right = [bytes[i], bytes[i + 1], bytes[i + 2]];
        let destination = [bytes[i + 7], bytes[i + 8], bytes[i + 9]];
        instructions.push(Instruction {
            left,
            right,
            op,
            destination,
        });
        i += 11;
    }
    instructions
}
