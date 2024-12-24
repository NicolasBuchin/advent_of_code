use std::collections::HashMap;

pub fn crossed_wires(input: &str) -> usize {
    let (mut operation_table, output_size) = parse(input);
    solve(&mut operation_table, output_size)
}

enum Op {
    And([u8; 3], [u8; 3]),
    Or([u8; 3], [u8; 3]),
    Xor([u8; 3], [u8; 3]),
    Value(bool),
}

fn solve(operation_table: &mut HashMap<[u8; 3], Op>, output_size: usize) -> usize {
    let mut output = 0;
    for i in 0..output_size {
        let key = [b'z', i.div_euclid(10) as u8 + b'0', i.rem_euclid(10) as u8 + b'0'];
        let value = get_value_of(key, operation_table);
        if value {
            output |= 1 << i;
        }
    }
    output
}

fn get_value_of(key: [u8; 3], operation_table: &mut HashMap<[u8; 3], Op>) -> bool {
    if let Some(op) = operation_table.remove(&key) {
        let value = match op {
            Op::And(left, right) => {
                let left_value = get_value_of(left, operation_table);
                let right_value = get_value_of(right, operation_table);
                left_value && right_value
            }
            Op::Or(left, right) => {
                let left_value = get_value_of(left, operation_table);
                let right_value = get_value_of(right, operation_table);
                left_value || right_value
            }
            Op::Xor(left, right) => {
                let left_value = get_value_of(left, operation_table);
                let right_value = get_value_of(right, operation_table);
                left_value ^ right_value
            }
            Op::Value(value) => value,
        };
        operation_table.insert(key, Op::Value(value));
        value
    } else {
        panic!("Key not found in operation_table");
    }
}

fn parse(input: &str) -> (HashMap<[u8; 3], Op>, usize) {
    let bytes = input.as_bytes();
    let mut operation_table = HashMap::new();
    let mut i = 0;

    while bytes[i] != b'\n' {
        let key = [bytes[i], bytes[i + 1], bytes[i + 2]];
        let value = Op::Value(bytes[i + 5] == b'1');
        operation_table.insert(key, value);
        i += 7;
    }

    i += 1;
    let mut output_size = 0;

    while i < bytes.len() {
        let value = match bytes[i + 4] {
            b'A' => {
                i += 15;
                Op::And(
                    [bytes[i - 15], bytes[i - 14], bytes[i - 13]],
                    [bytes[i - 7], bytes[i - 6], bytes[i - 5]],
                )
            }
            b'O' => {
                i += 14;
                Op::Or(
                    [bytes[i - 14], bytes[i - 13], bytes[i - 12]],
                    [bytes[i - 7], bytes[i - 6], bytes[i - 5]],
                )
            }
            b'X' => {
                i += 15;
                Op::Xor(
                    [bytes[i - 15], bytes[i - 14], bytes[i - 13]],
                    [bytes[i - 7], bytes[i - 6], bytes[i - 5]],
                )
            }
            _ => unreachable!(),
        };
        let key = [bytes[i], bytes[i + 1], bytes[i + 2]];
        if key[0] == b'z' {
            let size = (bytes[i + 1] - b'0') as usize * 10 + (bytes[i + 2] - b'0') as usize;
            if size > output_size {
                output_size = size;
            }
        }
        operation_table.insert(key, value);
        i += 4;
    }

    (operation_table, output_size + 1)
}
