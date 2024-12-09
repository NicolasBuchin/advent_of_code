pub fn disk_fragmenter(input: &str) -> usize {
    let mut input_clone = input.to_owned();
    let bytes: &mut [u8] = unsafe { input_clone.as_bytes_mut() };
    let mut checksum = 0;
    let mut idx = bytes[0] as usize - 0x30;

    let mut i = 1;

    let mut j = input.len() - 1;
    let mut v = j >> 1;

    loop {
        if i > j {
            break;
        }
        if bytes[i] - 0x30 == 0 {
            i += 1;
            let v = i >> 1;
            for _ in 0..bytes[i] - 0x30 {
                checksum += idx * v;
                idx += 1;
            }

            i += 1;
            continue;
        }
        if bytes[j] - 0x30 == 0 {
            j -= 2;
            v = j >> 1;
            continue;
        }

        checksum += idx * v;
        idx += 1;
        bytes[i] -= 1;
        bytes[j] -= 1;
    }

    checksum
}
