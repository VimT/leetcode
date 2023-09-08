//! 树中可以形成回文的路径数

use std::collections::HashMap;

pub fn count_palindrome_paths(parent: Vec<i32>, s: String) -> i64 {
    let len = parent.len();
    let mut g = vec![vec![]; len];
    for ((parent, i), &ch) in parent.into_iter().zip(0..).zip(s.as_bytes()) {
        if parent >= 0 {
            g[parent as usize].push((i, ch - b'a'));
        }
    }
    fn dfs(g: &Vec<Vec<(usize, u8)>>, u: usize, mask: i32, map: &mut HashMap<i32, i64>) {
        *map.entry(mask).or_default() += 1;
        for &(v, ch) in &g[u] {
            dfs(g, v, mask ^ (1 << ch), map)
        }
    }
    let mut map: HashMap<i32, i64> = HashMap::new();
    dfs(&g, 0, 0, &mut map);
    let mut result = 0;
    for (&mask, &cnt) in &map {
        result += cnt * (cnt - 1);
        for i in 0..26 {
            result += cnt * map.get(&(mask ^ (1 << i))).copied().unwrap_or(0);
        }
    }
    result / 2
}

fn main() {
    fn test(func: fn(parent: Vec<i32>, s: String) -> i64) {
        assert_eq!(func(vec![-1, 0, 0, 1, 1, 2], String::from("acaabc")), 8);
        assert_eq!(func(vec![-1, 0, 0, 0, 0], String::from("aaaaa")), 10);
    }
    test(count_palindrome_paths);
}
