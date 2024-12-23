pub fn lan_party(input: &str) -> usize {
    let mut connection_table = [[false; 26 * 26]; 26 * 26];
    parse(input, &mut connection_table);
    find_triples_with_t(&mut connection_table)
}

fn parse(input: &str, connection_table: &mut [[bool; 26 * 26]]) {
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let left = (bytes[i] - b'a') as usize * 26 + (bytes[i + 1] - b'a') as usize;
        let right = (bytes[i + 3] - b'a') as usize * 26 + (bytes[i + 4] - b'a') as usize;
        connection_table[left][right] = true;
        connection_table[right][left] = true;
        i += 6;
    }
}

fn find_triples_with_t(connection_table: &mut [[bool; 26 * 26]]) -> usize {
    let mut triples_with_t = 0;
    for t in 0..26 {
        let t = 19 * 26 + t;
        for x in 0..26 * 26 {
            if connection_table[t][x] {
                for y in 0..26 * 26 {
                    if connection_table[x][y] && connection_table[y][t] {
                        // println!("{}-{}-{}", translate(t), translate(x), translate(y));
                        triples_with_t += 1;
                    }
                }
            }
            connection_table[x][t] = false;
        }
    }
    triples_with_t
}

// fn translate(n: usize) -> String {
//     String::from_utf8(vec![n.div_euclid(26) as u8 + b'a', n.rem_euclid(26) as u8 + b'a']).unwrap()
// }
