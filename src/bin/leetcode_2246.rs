//! 相邻字符不同的最长路径

pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
    fn dfs(children: &Vec<Vec<usize>>, s: &[u8], cur: usize, result: &mut i32) -> i32 {
        let len = children[cur].len();
        if len == 0 { return 1; }
        let mut max_len1 = 0;
        let mut max_len2 = 0;
        for &child in &children[cur] {
            let sub = dfs(children, s, child, result);
            if s[child] != s[cur] {
                if sub > max_len1 {
                    max_len2 = max_len1;
                    max_len1 = sub;
                } else if sub > max_len2 {
                    max_len2 = sub;
                }
            }
        }
        *result = (*result).max(1 + max_len1 + max_len2);
        return max_len1 + 1;
    }
    let s = s.as_bytes();
    let len = s.len();
    let mut children = vec![vec![]; len];
    for (i, &p) in parent.iter().enumerate() {
        if p >= 0 {
            children[p as usize].push(i);
        }
    }
    let mut result = 1;
    let single = dfs(&children, s, 0, &mut result);
    result = result.max(single);
    result
}

fn main() {
    assert_eq!(longest_path(vec![-1, 0], String::from("mm")), 1);
    assert_eq!(longest_path(vec![-1, 0, 0, 1, 1, 2], String::from("abacbe")), 3);
    assert_eq!(longest_path(vec![-1, 0, 0, 0], String::from("aabc")), 3);
}
