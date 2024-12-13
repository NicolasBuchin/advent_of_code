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

    let mut map = [(false, false, false, false); 141 * 140];

    for i in 0..bytes.len() {
        let value = bytes[i];
        if value > b'Z' || value == b'\n' {
            continue;
        }
        let n = map_garden_area(value, i, bytes, line_size, &mut map);
        let w = count_walls(&map, line_size);
        println!("{} {}", n, w);
        total += n * w;
        map = [(false, false, false, false); 141 * 140];
    }

    total
}

fn map_garden_area(
    value: u8,
    i: usize,
    bytes: &mut [u8],
    line_size: usize,
    map: &mut [(bool, bool, bool, bool)],
) -> usize {
    bytes[i] += 32;
    let rem = i.rem_euclid(line_size);
    let (mut fu, mut fd, mut fl, mut fr) = (false, false, false, false);
    let mut num = 1;
    if i >= line_size {
        if bytes[i - line_size] == value {
            num += map_garden_area(value, i - line_size, bytes, line_size, map);
        } else if bytes[i - line_size] != bytes[i] {
            fu |= true;
        }
    } else {
        fu |= true;
    }
    if i < bytes.len() - line_size {
        if bytes[i + line_size] == value {
            num += map_garden_area(value, i + line_size, bytes, line_size, map);
        } else if bytes[i + line_size] != bytes[i] {
            fd |= true;
        }
    } else {
        fd |= true;
    }
    if rem > 0 {
        if bytes[i - 1] == value {
            num += map_garden_area(value, i - 1, bytes, line_size, map);
        } else if bytes[i - 1] != bytes[i] {
            fl |= true;
        }
    } else {
        fl |= true;
    }
    if rem < line_size - 1 {
        if bytes[i + 1] == value {
            num += map_garden_area(value, i + 1, bytes, line_size, map);
        } else if bytes[i + 1] != bytes[i] {
            fr |= true;
        }
    } else {
        fr |= true;
    }
    map[i] = (fu, fd, fl, fr);
    num
}
fn count_walls(map: &[(bool, bool, bool, bool)], line_size: usize) -> usize {
    let mut wall_count = 0;

    // Horizontal walls (above)
    for row in 0..map.len() / line_size {
        let mut in_wall = false;
        for col in 0..line_size {
            let index = row * line_size + col;
            if map[index].0 {
                // Check for a wall above
                if !in_wall {
                    wall_count += 1;
                    in_wall = true;
                }
            } else {
                in_wall = false;
            }
        }
    }

    // Horizontal walls (below)
    for row in 0..map.len() / line_size {
        let mut in_wall = false;
        for col in 0..line_size {
            let index = row * line_size + col;
            if map[index].1 {
                // Check for a wall below
                if !in_wall {
                    wall_count += 1;
                    in_wall = true;
                }
            } else {
                in_wall = false;
            }
        }
    }

    // Vertical walls (left)
    for col in 0..line_size {
        let mut in_wall = false;
        for row in 0..map.len() / line_size {
            let index = row * line_size + col;
            if map[index].2 {
                // Check for a wall to the left
                if !in_wall {
                    wall_count += 1;
                    in_wall = true;
                }
            } else {
                in_wall = false;
            }
        }
    }

    // Vertical walls (right)
    for col in 0..line_size {
        let mut in_wall = false;
        for row in 0..map.len() / line_size {
            let index = row * line_size + col;
            if map[index].3 {
                // Check for a wall to the right
                if !in_wall {
                    wall_count += 1;
                    in_wall = true;
                }
            } else {
                in_wall = false;
            }
        }
    }

    wall_count
}
