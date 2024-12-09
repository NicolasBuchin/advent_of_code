pub fn disk_fragmenter(input: &str) -> usize {
    let mut input_clone = input.to_owned();
    let bytes: &mut [u8] = unsafe { input_clone.as_bytes_mut() };
    let mut mem = [0usize; 50];
    let mut idx = bytes[0] as usize - 0x30;

    let mut i = 1;

    let mut j = input.len() - 1;

    loop {
        if i > j {
            break;
        }
        if bytes[i] - 0x30 == 0 {
            i += 2;
            // todo
            continue;
        }
        if bytes[j] - 0x30 == 0 {
            j -= 2;
            continue;
        }

        mem[idx] = j >> 1;
        idx += 1;
        bytes[i] -= 1;
        bytes[j] -= 1;
    }

    println!("{:?}", &mem[0..idx]);

    0
}
