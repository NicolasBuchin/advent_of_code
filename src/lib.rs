pub fn lan_party(input: &str) -> String {
    let connection_table = parse(input);

    let triples = find_triples(&connection_table);

    let best_set = find_larget_connection_set(triples, &connection_table);

    translate(best_set)
}

fn parse(input: &str) -> [[bool; 26 * 26]; 26 * 26] {
    let bytes = input.as_bytes();
    let mut connection_table = [[false; 26 * 26]; 26 * 26];
    let mut i = 0;

    while i < bytes.len() {
        let left = (bytes[i] - b'a') as usize * 26 + (bytes[i + 1] - b'a') as usize;
        let right = (bytes[i + 3] - b'a') as usize * 26 + (bytes[i + 4] - b'a') as usize;
        connection_table[left][right] = true;
        connection_table[right][left] = true;
        i += 6;
    }
    connection_table
}

fn find_triples(connection_table: &[[bool; 26 * 26]; 26 * 26]) -> Vec<Vec<usize>> {
    let mut connection_table = *connection_table;
    let mut triples_with_t = Vec::new();
    for x in 0..26 * 26 {
        for y in 0..26 * 26 {
            if connection_table[x][y] {
                for z in 0..26 * 26 {
                    if connection_table[y][z] && connection_table[z][x] {
                        triples_with_t.push(vec![x, y, z]);
                    }
                }
            }
            connection_table[y][x] = false;
        }
    }
    triples_with_t
}

fn find_larget_connection_set(sets: Vec<Vec<usize>>, connection_table: &[[bool; 26 * 26]]) -> Vec<usize> {
    let mut best_sets: Vec<Vec<usize>> = Vec::new();
    for s1 in sets {
        let mut to_add = true;
        for s2 in &mut best_sets {
            if let Some(set) = merge_sets(&s1, s2, connection_table) {
                *s2 = set;
                to_add = false;
                break;
            }
        }
        if to_add {
            best_sets.push(s1);
        }
    }

    let mut best = &best_sets[0];
    for set in &best_sets {
        if set.len() > best.len() {
            best = set;
        }
    }

    best.clone()
}

fn merge_sets(set1: &[usize], set2: &[usize], connection_table: &[[bool; 26 * 26]]) -> Option<Vec<usize>> {
    for &x in set1 {
        for &y in set2 {
            if x != y && !connection_table[x][y] {
                return None;
            }
        }
    }

    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < set1.len() && j < set2.len() {
        let x = set1[i];
        let y = set2[j];
        match x.cmp(&y) {
            std::cmp::Ordering::Less => {
                result.push(x);
                i += 1;
            }
            std::cmp::Ordering::Equal => {
                result.push(x);
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Greater => {
                result.push(y);
                j += 1;
            }
        }
    }
    while i < set1.len() {
        result.push(set1[i]);
        i += 1;
    }
    while j < set2.len() {
        result.push(set2[j]);
        j += 1;
    }

    Some(result)
}

fn translate(set: Vec<usize>) -> String {
    let mut ns = Vec::with_capacity(set.len() * 3);
    for &n in &set {
        if n != set[0] {
            ns.push(b',');
        }
        ns.push(n.div_euclid(26) as u8 + b'a');
        ns.push(n.rem_euclid(26) as u8 + b'a');
    }
    String::from_utf8(ns).unwrap()
}
