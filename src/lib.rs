pub fn garden_groups(input: &str) -> usize {
    let mut input_clone = input.to_owned();
    let bytes = unsafe { input_clone.as_bytes_mut() };

    let line_size = {
        let mut i = 0;
        while bytes[i] != b'\n' {
            i += 1;
        }
        i + 1
    };

    let mut total = 0;
    for i in 0..bytes.len() {
        let value = bytes[i];
        if value > b'Z' || value == b'\n' {
            continue;
        }
        let (n, w) = garden_area(value, i, bytes, line_size);
        total += n * w;
    }

    total
}

fn garden_area(value: u8, i: usize, bytes: &mut [u8], line_size: usize) -> (usize, usize) {
    bytes[i] += 27;
    let (mut num, mut walls) = (1, 0);
    if i >= line_size {
        if bytes[i - line_size] == value {
            let (n, w) = garden_area(value, i - line_size, bytes, line_size);
            num += n;
            walls += w;
        } else if bytes[i - line_size] != bytes[i] {
            walls += 1;
        }
    } else {
        walls += 1;
    }
    if i < bytes.len() - line_size {
        if bytes[i + line_size] == value {
            let (n, w) = garden_area(value, i + line_size, bytes, line_size);
            num += n;
            walls += w;
        } else if bytes[i + line_size] != bytes[i] {
            walls += 1;
        }
    } else {
        walls += 1;
    }
    let rem = i.rem_euclid(line_size);
    if rem > 0 {
        if bytes[i - 1] == value {
            let (n, w) = garden_area(value, i - 1, bytes, line_size);
            num += n;
            walls += w;
        } else if bytes[i - 1] != bytes[i] {
            walls += 1;
        }
    } else {
        walls += 1;
    }
    if rem < line_size - 1 {
        if bytes[i + 1] == value {
            let (n, w) = garden_area(value, i + 1, bytes, line_size);
            num += n;
            walls += w;
        } else if bytes[i + 1] != bytes[i] {
            walls += 1;
        }
    } else {
        walls += 1;
    }
    (num, walls)
}
