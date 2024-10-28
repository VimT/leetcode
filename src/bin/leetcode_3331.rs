//! 修改后子树的大小

pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
    let s = s.as_bytes();
    let n = parent.len();
    let mut g = vec![vec![]; n];
    for (u, &parent) in parent.iter().enumerate() {
        if parent >= 0 {
            g[parent as usize].push(u);
        }
    }

    fn dfs_new_parent(g: &Vec<Vec<usize>>, s: &[u8], u: usize, latest: &mut [i32; 26], new_parent: &mut Vec<i32>) {
        let ch = (s[u] - b'a') as usize;
        let before = latest[ch];
        if before != -1 {
            new_parent[u] = before;
        }
        latest[ch] = u as i32;
        for &v in &g[u] {
            dfs_new_parent(g, s, v, latest, new_parent);
        }
        latest[ch] = before;
    }
    let mut new_parent = parent.clone();
    dfs_new_parent(&g, s, 0, &mut [-1; 26], &mut new_parent);

    let mut g = vec![vec![]; n];
    for (u, &parent) in new_parent.iter().enumerate() {
        if parent >= 0 {
            g[parent as usize].push(u);
        }
    }
    let mut result = vec![0; n];
    fn dfs(g: &Vec<Vec<usize>>, u: usize, result: &mut Vec<i32>) -> i32 {
        let mut cnt = 1;
        for &v in &g[u] {
            cnt += dfs(g, v, result);
        }
        result[u] = cnt;
        cnt
    }
    dfs(&g, 0, &mut result);
    result
}

fn main() {
    fn test(func: fn(parent: Vec<i32>, s: String) -> Vec<i32>) {
        assert_eq!(func(vec![-1, 0, 0, 1, 1, 1], String::from("abaabc")), vec![6, 3, 1, 1, 1, 1]);
        assert_eq!(func(vec![-1, 0, 4, 0, 1], String::from("abbba")), vec![5, 2, 1, 1, 1]);
    }
    test(find_subtree_sizes);
}
