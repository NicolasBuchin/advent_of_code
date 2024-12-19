#[derive(Debug)]
#[allow(dead_code)]
struct TowelTreeNode {
    w: Option<Box<TowelTreeNode>>,
    b: Option<Box<TowelTreeNode>>,
    u: Option<Box<TowelTreeNode>>,
    r: Option<Box<TowelTreeNode>>,
    g: Option<Box<TowelTreeNode>>,
    is_final: bool,
}

#[allow(dead_code)]
impl TowelTreeNode {
    fn new() -> Self {
        TowelTreeNode {
            w: None,
            b: None,
            u: None,
            r: None,
            g: None,
            is_final: false,
        }
    }

    fn insert(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            self.is_final = true;
            return;
        }

        let child = match bytes[0] {
            b'w' => &mut self.w,
            b'b' => &mut self.b,
            b'u' => &mut self.u,
            b'r' => &mut self.r,
            b'g' => &mut self.g,
            _ => panic!("Not w, b, u, r or g!"),
        };

        if child.is_none() {
            *child = Some(Box::new(TowelTreeNode::new()));
        }

        if let Some(node) = child {
            node.insert(&bytes[1..]);
        }
    }

    fn contains(&self, bytes: &[u8]) -> bool {
        if bytes.is_empty() {
            return self.is_final;
        }

        let child = match bytes[0] {
            b'w' => &self.w,
            b'b' => &self.b,
            b'u' => &self.u,
            b'r' => &self.r,
            b'g' => &self.g,
            _ => panic!("Not w, b, u, r or g!"),
        };

        match child {
            Some(node) => node.contains(&bytes[1..]),
            None => false,
        }
    }

    fn find_end_positions(&self, bytes: &[u8], current_pos: usize, results: &mut Vec<usize>) {
        if self.is_final {
            results.push(current_pos);
        }
        if bytes.is_empty() {
            return;
        }

        let child = match bytes[0] {
            b'w' => &self.w,
            b'b' => &self.b,
            b'u' => &self.u,
            b'r' => &self.r,
            b'g' => &self.g,
            _ => panic!("Not w, b, u, r or g!"),
        };

        if let Some(node) = child {
            node.find_end_positions(&bytes[1..], current_pos + 1, results);
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TowelTree {
    root: TowelTreeNode,
}

#[allow(dead_code)]
impl TowelTree {
    pub fn new() -> Self {
        TowelTree {
            root: TowelTreeNode::new(),
        }
    }

    pub fn insert(&mut self, bytes: &[u8]) {
        self.root.insert(bytes);
    }

    pub fn contains(&self, bytes: &[u8]) -> bool {
        self.root.contains(bytes)
    }

    fn find_end_positions(&self, bytes: &[u8]) -> Vec<usize> {
        let mut results = Vec::new();
        self.root.find_end_positions(bytes, 0, &mut results);
        results
    }

    pub fn is_composable(&self, bytes: &[u8]) -> bool {
        let mut dp = vec![false; bytes.len() + 1];
        dp[0] = true;

        for start in 0..bytes.len() {
            if !dp[start] {
                continue;
            }

            let positions = self.find_end_positions(&bytes[start..]);

            for &end_offset in &positions {
                let end = start + end_offset;
                if end <= bytes.len() {
                    dp[end] = true;
                }
            }
        }

        dp[bytes.len()]
    }

    pub fn count_compositions(&self, bytes: &[u8]) -> usize {
        let mut dp = vec![0_usize; bytes.len() + 1];
        dp[0] = 1;

        for start in 0..bytes.len() {
            if dp[start] == 0 {
                continue;
            }

            let positions = self.find_end_positions(&bytes[start..]);

            for &end_offset in &positions {
                let end = start + end_offset;
                if end <= bytes.len() {
                    dp[end] = dp[end].saturating_add(dp[start]);
                }
            }
        }
        dp[bytes.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = TowelTree::new();
        assert!(!tree.contains(b"w"));
        assert!(!tree.contains(b""));
    }

    #[test]
    fn test_single_insertions() {
        let mut tree = TowelTree::new();
        tree.insert(b"w");

        assert!(tree.contains(b"w"));
        assert!(!tree.contains(b"b"));
        assert!(!tree.contains(b"wu"));
    }

    #[test]
    fn test_multiple_insertions() {
        let mut tree = TowelTree::new();
        tree.insert(b"w");
        tree.insert(b"wu");
        tree.insert(b"wub");

        assert!(tree.contains(b"w"));
        assert!(tree.contains(b"wu"));
        assert!(tree.contains(b"wub"));
        assert!(!tree.contains(b"wubg"));
    }

    #[test]
    fn test_all_characters() {
        let mut tree = TowelTree::new();
        tree.insert(b"w");
        tree.insert(b"b");
        tree.insert(b"u");
        tree.insert(b"r");
        tree.insert(b"g");

        assert!(tree.contains(b"w"));
        assert!(tree.contains(b"b"));
        assert!(tree.contains(b"u"));
        assert!(tree.contains(b"r"));
        assert!(tree.contains(b"g"));
    }

    #[test]
    fn test_find_end_positions() {
        let mut tree = TowelTree::new();
        tree.insert(b"w");
        tree.insert(b"wu");
        tree.insert(b"wub");

        let positions = tree.find_end_positions(b"wub");
        assert_eq!(positions, vec![1, 2, 3]);
    }

    #[test]
    fn test_is_composable() {
        let mut tree = TowelTree::new();
        tree.insert(b"w");
        tree.insert(b"wu");
        tree.insert(b"ub");

        assert!(tree.is_composable(b"wub"));
        assert!(!tree.is_composable(b"wug"));
    }

    #[test]
    fn test_count_compositions() {
        let mut tree = TowelTree::new();
        tree.insert(b"w");
        tree.insert(b"wu");
        tree.insert(b"ub");
        tree.insert(b"uub");

        assert_eq!(tree.count_compositions(b"wuub"), 2);
        assert_eq!(tree.count_compositions(b"wug"), 0);
    }

    #[test]
    fn test_overlapping_patterns() {
        let mut tree = TowelTree::new();
        tree.insert(b"w");
        tree.insert(b"wb");
        tree.insert(b"b");

        assert_eq!(tree.count_compositions(b"wb"), 2);
        assert!(tree.is_composable(b"wb"));
    }

    #[test]
    #[should_panic(expected = "Not w, b, u, r or g!")]
    fn test_invalid_character() {
        let mut tree = TowelTree::new();
        tree.insert(b"x");
    }

    #[test]
    fn test_long_sequence() {
        let mut tree = TowelTree::new();
        tree.insert(b"w");
        tree.insert(b"u");
        tree.insert(b"b");
        tree.insert(b"wu");
        tree.insert(b"wub");
        tree.insert(b"ub");
        tree.insert(b"bw");

        assert!(tree.is_composable(b"wubwub"));
        assert_eq!(tree.count_compositions(b"wubwub"), 20);
    }
}
