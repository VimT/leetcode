//! 金字塔转换矩阵

use leetcode::svec;

pub fn pyramid_transition(bottom: String, mut allowed: Vec<String>) -> bool {
    fn dfs(bottom: &Vec<u8>, cur: &mut Vec<u8>, allowed: &Vec<&[u8]>, map: &Vec<Vec<Vec<usize>>>) -> bool {
        if bottom.len() == 1 { return true; }
        let len = cur.len();
        if len == bottom.len() - 1 {
            return dfs(&cur, &mut vec![], allowed, map);
        }
        let can_use_idxs = &map[(bottom[len] - b'A') as usize][(bottom[len + 1] - b'A') as usize];
        for i in 0..can_use_idxs.len() {
            let allow_idx = can_use_idxs[i];
            cur.push(allowed[allow_idx][2]);
            if dfs(bottom, cur, allowed, map) { return true; }
            cur.pop();
        }
        false
    }
    let len = allowed.len();
    allowed.sort_unstable();
    let mut map = vec![vec![vec![]; 7]; 7];
    let mut allow_bytes = vec![];
    for i in 0..len {
        if i > 0 && allowed[i] == allowed[i - 1] { continue; }
        allow_bytes.push(allowed[i].as_bytes());
    }
    for (i, v) in allow_bytes.iter().enumerate() {
        map[(v[0] - b'A') as usize][(v[1] - b'A') as usize].push(i);
    }
    let high = bottom.len() - 1;
    if (1 + high) * high / 2 > len { return false; }
    dfs(&bottom.into_bytes(), &mut vec![], &allow_bytes, &map)
}

/// 方法不正确
pub fn pyramid_transition_bit(bottom: String, allowed: Vec<String>) -> bool {
    let mut t = vec![vec![0; 1 << 7]; 1 << 7];
    for allow in allowed {
        let s = allow.as_bytes();
        let (u, v, w) = (1 << s[0] - b'A', 1 << s[1] - b'A', 1 << s[2] - b'A');
        for i in 0..1 << 7 {
            if i & u > 0 {
                for j in 0..1 << 7 {
                    if j & v > 0 {
                        t[i][j] |= w;
                    }
                }
            }
        }
    }
    let mut state: Vec<usize> = bottom.as_bytes().iter().map(|x| 1 << (*x - b'A')).collect();
    while state.len() > 1 {
        for i in 0..state.len() - 1 {
            state[i] = t[state[i]][state[i + 1]]
        }
        state.pop();
    }
    state[0] > 0
}

fn main() {
    assert_eq!(pyramid_transition(String::from("AAAA"), svec!["AAB", "AAC", "BCD", "BBE", "DEF"]), false);
    assert_eq!(pyramid_transition(String::from("BCD"), svec!["ACC", "ACB", "ABD", "DAA", "BDC", "BDB", "DBC", "BBD", "BBC", "DBD", "BCC", "CDD", "ABA", "BAB", "DDC", "CCD", "DDA", "CCA", "DDD"]), true);
    assert_eq!(pyramid_transition(String::from("CCC"), svec!["CBB", "ACB", "ABD", "CDB", "BDC", "CBC", "DBA", "DBB", "CAB", "BCB", "BCC", "BAA", "CCD", "BDD", "DDD", "CCA", "CAA", "CCC", "CCB"]), true);
    assert_eq!(pyramid_transition(String::from("BCD"), svec!["BCC", "CDE", "CEA", "FFF"]), true);
    assert_eq!(pyramid_transition(String::from("AABA"), svec!["AAA", "AAB", "ABA", "ABB", "BAC"]), false);
}
