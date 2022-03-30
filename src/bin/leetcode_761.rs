//! 特殊的二进制序列

/// 转换为括号字符串
pub fn make_largest_special(s: String) -> String {
    let mut s = s.into_bytes();
    let len = s.len();
    let mut end = vec![0; len];
    let mut stack = vec![];
    for i in 0..len {
        if s[i] == b'1' {
            stack.push(i);
        } else {
            let start = stack.pop().unwrap();
            end[start] = i;
        }
    }

    fn dfs(s: &mut [u8], l: usize, r: usize, end: &Vec<usize>) {
        if r == l + 1 || l > r { return; }
        let mut duans = vec![];
        let mut p = l;
        while p < r {
            let e = end[p];
            dfs(s, p + 1, e - 1, end);
            duans.push(s[p..=e].to_vec());
            p = e + 1;
        }
        duans.sort_unstable();
        p = l;
        for duan in duans.iter().rev() {
            for &ch in duan {
                s[p] = ch;
                p += 1;
            }
        }
    }
    dfs(&mut s, 0, len - 1, &end);
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(make_largest_special(String::from("11011000")), String::from("11100100"));
    assert_eq!(make_largest_special(String::from("10")), String::from("10"));
}
