use towel_tree::TowelTree;
mod towel_tree;

pub fn linen_layout(input: &str) -> usize {
    let (tree, words) = parse(input);

    let mut result = 0;

    words.iter().for_each(|&word| {
        result += tree.count_compositions(word);
    });

    result
}

fn parse(input: &str) -> (TowelTree, Vec<&[u8]>) {
    let bytes = input.as_bytes();
    let mut tree = TowelTree::new();
    let mut start = 0;

    let mut i = 0;
    loop {
        while bytes[i] != b',' && bytes[i] != b'\n' {
            i += 1;
        }
        tree.insert(&bytes[start..i]);
        if bytes[i] == b'\n' {
            break;
        }
        i += 2;
        start = i;
    }

    i += 2;
    start = i;

    let mut words = Vec::new();

    while i < bytes.len() {
        if bytes[i] == b'\n' {
            words.push(&bytes[start..i]);
            start = i + 1;
        }
        i += 1;
    }

    (tree, words)
}
