use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const SEARCH_DEPTH: usize = 75;

pub fn plutonian_pebbles(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut search_map = [[0usize; SEARCH_DEPTH + 1]; 10];

    search_map[0][0] = 1;
    search_map[0][1] = 1;
    search_map[0][2] = 1;
    search_map[0][3] = 2;
    search_map[0][4] = 4;
    for x in 1..5 {
        search_map[x][0] = 1;
        search_map[x][1] = 1;
        search_map[x][2] = 2;
        search_map[x][3] = 4;
        search_map[x][4] = 4;
    }
    for x in 5..10 {
        search_map[x][0] = 1;
        search_map[x][1] = 1;
        search_map[x][2] = 1;
        search_map[x][3] = 2;
        search_map[x][4] = 4;
    }

    for x in 5..=SEARCH_DEPTH {
        search_map[0][x] = 2 * search_map[2][x - 4] + search_map[0][x - 4] + search_map[4][x - 4];
        search_map[1][x] = 2 * search_map[2][x - 3] + search_map[0][x - 3] + search_map[4][x - 3];
        search_map[2][x] = 2 * search_map[4][x - 3] + search_map[0][x - 3] + search_map[8][x - 3];
        search_map[3][x] = search_map[6][x - 3]
            + search_map[0][x - 3]
            + search_map[7][x - 3]
            + search_map[2][x - 3];
        search_map[4][x] = search_map[8][x - 3]
            + search_map[0][x - 3]
            + search_map[9][x - 3]
            + search_map[6][x - 3];
        search_map[5][x] = 2 * search_map[0][x - 5]
            + 2 * search_map[2][x - 5]
            + search_map[4][x - 5]
            + 3 * search_map[8][x - 5];
        search_map[6][x] = search_map[2][x - 5]
            + 2 * search_map[4][x - 5]
            + 2 * search_map[5][x - 5]
            + search_map[6][x - 5]
            + search_map[7][x - 5]
            + search_map[9][x - 5];
        search_map[7][x] = search_map[0][x - 5]
            + 2 * search_map[2][x - 5]
            + search_map[3][x - 5]
            + 2 * search_map[6][x - 5]
            + search_map[7][x - 5]
            + search_map[8][x - 5];
        search_map[8][x] = 2 * search_map[2][x - 5]
            + search_map[3][x - 5]
            + search_map[6][x - 5]
            + 2 * search_map[7][x - 5]
            + search_map[8][x - 4];
        search_map[9][x] = search_map[1][x - 5]
            + search_map[3][x - 5]
            + search_map[4][x - 5]
            + 2 * search_map[6][x - 5]
            + 2 * search_map[8][x - 5]
            + search_map[9][x - 5];
    }

    let mut stones = [0usize; 10];
    let mut stones_len = 0;

    let mut i = 0;

    loop {
        while bytes[i] <= b'9' && bytes[i] >= b'0' {
            stones[stones_len] = stones[stones_len] * 10 + bytes[i] as usize - 0x30;
            i += 1;
        }
        stones_len += 1;
        if bytes[i] == b'\n' {
            break;
        }
        i += 1;
    }

    stones[0..stones_len]
        .par_iter()
        .map(|stone| transform_count_search(*stone, SEARCH_DEPTH, &search_map))
        .sum()
}

fn transform_count_search(
    value: usize,
    mut depth: usize,
    search_map: &[[usize; SEARCH_DEPTH + 1]; 10],
) -> usize {
    if depth == 0 {
        return 1;
    }

    if value.div_euclid(10) == 0 {
        return search_map[value][depth];
    }

    depth -= 1;

    if value == 0 {
        return transform_count_search(1, depth, search_map);
    }

    let length = value.checked_ilog10().unwrap_or(0) + 1;
    if length.rem_euclid(2) == 0 {
        let middle = 10usize.pow(length.div_euclid(2));
        if SEARCH_DEPTH - depth < 15 {
            let values = [value.div_euclid(middle), value.rem_euclid(middle)];
            values
                .par_iter()
                .map(|value| transform_count_search(*value, depth, search_map))
                .sum()
        } else {
            let left = value.div_euclid(middle);
            let right = value.rem_euclid(middle);
            return transform_count_search(left, depth, search_map)
                + transform_count_search(right, depth, search_map);
        }
    } else {
        return transform_count_search(value * 2024, depth, search_map);
    }
}
