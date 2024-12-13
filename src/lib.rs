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

    let mut walls = [(false, false, false, false); 141 * 140];

    for i in 0..bytes.len() {
        let value = bytes[i];
        if value > b'Z' || value == b'\n' {
            continue;
        }
        let (n, w) = garden_area(value, i, bytes, line_size, &mut walls);
        total += n * w;
    }

    total
}

fn garden_area(
    value: u8,
    i: usize,
    bytes: &mut [u8],
    line_size: usize,
    walls: &mut [(bool, bool, bool, bool)],
) -> (usize, usize) {
    bytes[i] += 26;
    let (mut num, mut walls_count) = (1, 0);
    let (mut fu, mut fd, mut fl, mut fr) = (false, false, false, false);
    let (mut nu, mut nd, mut nl, mut nr) = (false, false, false, false);
    if i >= line_size {
        if bytes[i - line_size] == value {
            let (n, w) = garden_area(value, i - line_size, bytes, line_size, walls);
            num += n;
            walls_count += w;
            nl |= walls[i - line_size].2;
            nr |= walls[i - line_size].3;
        } else if bytes[i - line_size] == bytes[i] {
            nl |= walls[i - line_size].2;
            nr |= walls[i - line_size].3;
        } else {
            walls_count += 1;
            fu |= true;
        }
    } else {
        walls_count += 1;
        fu |= true;
    }
    if i < bytes.len() - line_size {
        if bytes[i + line_size] == value {
            let (n, w) = garden_area(value, i + line_size, bytes, line_size, walls);
            num += n;
            walls_count += w;
            nl |= walls[i + line_size].2;
            nr |= walls[i + line_size].3;
        } else if bytes[i + line_size] == bytes[i] {
            nl |= walls[i + line_size].2;
            nr |= walls[i + line_size].3;
        } else {
            walls_count += 1;
            fd |= true;
        }
    } else {
        walls_count += 1;
        fd |= true;
    }
    let rem = i.rem_euclid(line_size);
    if rem > 0 {
        if bytes[i - 1] == value {
            let (n, w) = garden_area(value, i - 1, bytes, line_size, walls);
            num += n;
            walls_count += w;
            nu |= walls[i - 1].0;
            nd |= walls[i - 1].1;
        } else if bytes[i - 1] == bytes[i] {
            nu |= walls[i - 1].0;
            nd |= walls[i - 1].1;
        } else {
            walls_count += 1;
            fl |= true;
        }
    } else {
        walls_count += 1;
        fl |= true;
    }
    if rem < line_size - 1 {
        if bytes[i + 1] == value {
            let (n, w) = garden_area(value, i + 1, bytes, line_size, walls);
            num += n;
            walls_count += w;
            nu |= walls[i + 1].0;
            nd |= walls[i + 1].1;
        } else if bytes[i + 1] == bytes[i] {
            nu |= walls[i + 1].0;
            nd |= walls[i + 1].1;
        } else {
            walls_count += 1;
            fr |= true;
        }
    } else {
        walls_count += 1;
        fr |= true;
    }
    if fu && nu {
        walls_count -= 1;
    }
    if fd && nd {
        walls_count -= 1;
    }
    if fl && nl {
        walls_count -= 1;
    }
    if fr && nr {
        walls_count -= 1;
    }
    walls[i] = (fu, fd, fl, fr);
    (num, walls_count)
}
