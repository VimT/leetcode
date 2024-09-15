//! 形成目标字符串需要的最少字符串数 I

pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    #[derive(Default, Debug)]
    pub struct Trie {
        children: [Option<Box<Trie>>; 26],
    }

    impl Trie {
        pub fn insert(&mut self, s: &[u8]) {
            let mut node = self;
            for &ch in s {
                let idx = (ch - b'a') as usize;
                node = node.children[idx].get_or_insert_with(Box::<Trie>::default);
            }
        }
    }
    let mut trie = Trie::default();
    for word in &words {
        trie.insert(word.as_bytes());
    }

    let s = target.as_bytes();
    let len = s.len();

    const INF: i32 = i32::MAX / 2;
    let mut dp = vec![INF; len + 1];
    dp[0] = 0;
    for i in 0..len {
        let mut node = &trie;
        for (&ch, j) in s[i..].iter().zip(1..) {
            if let Some(child) = &node.children[(ch - b'a') as usize] {
                node = child;
                dp[i + j] = dp[i + j].min(dp[i] + 1);
            } else { break; }
        }
    }
    if dp[len] >= INF { -1 } else { dp[len] }
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>, target: String) -> i32) {
        assert_eq!(func(svec!["abc","aaaaa","bcdef"], String::from("aabcdabc")), 3);
        assert_eq!(func(svec!["abababab","ab"], String::from("ababaababa")), 2);
        assert_eq!(func(svec!["abcdef"], String::from("xyz")), -1);
    }
    test(min_valid_strings);
}
