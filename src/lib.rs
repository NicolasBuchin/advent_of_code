pub fn chronospatial_computer(input: &str) -> usize {
    let (mut ra, mut rb, mut rc, memory) = parse(input);
    let mut ptr = 0;

    let mut result = 0;

    while ptr < memory.len() - 1 {
        let opcode = memory[ptr];
        let operand = memory[ptr + 1];
        let prev_ptr = ptr;
        let processed = process(opcode, operand, &mut ra, &mut rb, &mut rc, &mut ptr);

        if let Some(value) = processed {
            result *= 10;
            result += value;
        }

        if ptr == prev_ptr {
            ptr += 2;
        }
    }

    result
}

fn combo(operand: usize, ra: &usize, rb: &usize, rc: &usize) -> usize {
    match operand {
        0_usize..=3_usize => operand,
        4 => *ra,
        5 => *rb,
        6 => *rc,
        _ => panic!(),
    }
}

fn process(
    opcode: usize,
    operand: usize,
    ra: &mut usize,
    rb: &mut usize,
    rc: &mut usize,
    ptr: &mut usize,
) -> Option<usize> {
    match opcode {
        0 => {
            *ra = ra.div_euclid(2usize.pow(combo(operand, ra, rb, rc) as u32));
            None
        }
        1 => {
            *rb ^= operand;
            None
        }
        2 => {
            *rb = combo(operand, ra, rb, rc).rem_euclid(8);
            None
        }
        3 => {
            if *ra == 0 {
                None
            } else {
                *ptr = operand;
                None
            }
        }
        4 => {
            *rb ^= *rc;
            None
        }
        5 => Some(combo(operand, ra, rb, rc).rem_euclid(8)),
        6 => {
            *rb = ra.div_euclid(2usize.pow(combo(operand, ra, rb, rc) as u32));
            None
        }
        7 => {
            *rc = ra.div_euclid(2usize.pow(combo(operand, ra, rb, rc) as u32));
            None
        }
        _ => panic!(),
    }
}

fn parse(input: &str) -> (usize, usize, usize, Vec<usize>) {
    let bytes = input.as_bytes();

    let mut ra = 0;
    let mut rb = 0;
    let mut rc = 0;

    let mut i = 12;

    while bytes[i] != b'\n' {
        ra *= 10;
        ra += bytes[i] as usize - 0x30;
        i += 1
    }
    i += 13;
    while bytes[i] != b'\n' {
        rb *= 10;
        rb += bytes[i] as usize - 0x30;
        i += 1
    }
    i += 13;
    while bytes[i] != b'\n' {
        rc *= 10;
        rc += bytes[i] as usize - 0x30;
        i += 1
    }
    i += 11;

    let mut memory = Vec::new();

    while i < bytes.len() {
        memory.push(bytes[i] as usize - 0x30);
        i += 2
    }

    (ra, rb, rc, memory)
}
