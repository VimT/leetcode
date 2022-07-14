//! 形成字符串的最短路径

/// next数组
pub fn shortest_way(source: String, target: String) -> i32 {
    let s = source.as_bytes();
    let t = target.as_bytes();
    let mut sch = [false; 26];
    for &ch in s {
        sch[(ch - b'a') as usize] = true;
    }
    let mut tch = [false; 26];
    for &ch in t {
        tch[(ch - b'a') as usize] = true;
    }
    for i in 0..26 {
        if tch[i] && !sch[i] { return -1; }
    }
    let len = s.len();
    let mut last = [len; 26];
    let mut next = vec![[0; 26]; len + 1];
    for i in (0..len).rev() {
        next[i] = last.clone();
        last[(s[i] - b'a') as usize] = i;
    }
    next[len] = last;
    let mut result = 1;
    let mut i = len;
    for &ch in t {
        i = next[i][(ch - b'a') as usize];
        if i == len {
            i = next[i][(ch - b'a') as usize];
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(source: String, target: String) -> i32) {
        assert_eq!(func(String::from("xyz"), String::from("xzyxz")), 3);
        assert_eq!(func(String::from("abc"), String::from("abcbc")), 2);
        assert_eq!(func(String::from("abc"), String::from("acdbc")), -1);
    }
    test(shortest_way);
}
