//! 删除星号以后字典序最小的字符串

pub fn clear_stars(s: String) -> String {
    let s = s.as_bytes();
    let mut q = vec![vec![]; 26];
    for (i, &ch) in s.iter().enumerate() {
        if ch == b'*' {
            for i in 0..26 {
                if !q[i].is_empty() {
                    q[i].pop();
                    break;
                }
            }
        } else {
            q[(ch - b'a') as usize].push(i);
        }
    }
    let mut result = Vec::with_capacity(s.len());
    for i in 0..26 {
        result.extend_from_slice(&q[i]);
    }
    result.sort_unstable();
    String::from_utf8(result.into_iter().map(|x| s[x]).collect()).unwrap()
}

/// 原地修改
pub fn clear_stars2(s: String) -> String {
    let mut s = s.into_bytes();
    let mut q = vec![vec![]; 26];
    for i in 0..s.len() {
        if s[i] == b'*' {
            for i in 0..26 {
                if let Some(j) = q[i].pop() {
                    s[j] = b'*';
                    break;
                }
            }
        } else {
            q[(s[i] - b'a') as usize].push(i);
        }
    }
    String::from_utf8(s.into_iter().filter(|x| *x != b'*').collect()).unwrap()
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("aaba*")), String::from("aab"));
        assert_eq!(func(String::from("abc")), String::from("abc"));
    }
    test(clear_stars);
    test(clear_stars2);
}
