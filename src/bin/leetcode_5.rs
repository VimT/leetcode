//! 最长回文子串


use std::cmp::Reverse;

pub fn longest_palindrome(s: String) -> String {
    fn expand(s: &Vec<char>, mut left: i32, mut right: i32) -> i32 {
        while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        return right - left - 1;
    }
    if s.len() <= 1 { return s; }
    let chars: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut end = 0;

    for i in 0..chars.len() {
        let len1 = expand(&chars, i as i32, i as i32);
        let len2 = expand(&chars, i as i32, (i + 1) as i32);
        let len = len1.max(len2);

        if len > (end - start + 1) as i32 {
            start = i - ((len - 1) / 2) as usize;
            end = i + (len / 2) as usize;
        }
    }
    s[start..=end].to_string()
}

/// 马拉车算法
pub fn longest_palindrome_manacher(s: String) -> String {
    // 预处理，插入 # 符号
    let mut pre = vec![b'^', b'#'];
    for &i in s.as_bytes() {
        pre.push(i);
        pre.push(b'#');
    }
    pre.push(b'$');
    let pre = unsafe { String::from_utf8_unchecked(pre) };
    let chars = pre.as_bytes();
    let n = chars.len();
    let mut p = vec![0; n]; //从每个字符中心扩展的长度
    let mut c = 0; // 上一个回文串的中心
    let mut r = 0; // 上一个回文串的最右点

    for i in 1..n - 1 {
        // 如果当前遍历的位置 处于上个回文串点的范围
        p[i] = if r > i {
            let i_mirror = 2 * c - i;
            (r - i).min(p[i_mirror]) // 防止对称位置超出R, 超出已知范围
        } else { 0 }; // 等于R
        while chars[i + 1 + p[i]] == chars[i - 1 - p[i]] { p[i] += 1; }
        // 如果超出上个回文串的范围，则更新为当前回文串
        if i + p[i] > r {
            c = i;
            r = i + p[i];
        }
    }

    // 找P的最大值
    let (max_len, Reverse(center_idx)) = p.into_iter().enumerate().map(|(i, v)| (v, Reverse(i))).max().unwrap();
    // 原始字符的 start = (i - p[i]) / 2;
    let start = (center_idx - max_len) / 2;
    return s[start..start + max_len].to_string();
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("babad")), String::from("bab"));
        assert_eq!(func(String::from("cbbd")), String::from("bb"));
    }
    test(longest_palindrome);
    test(longest_palindrome_manacher);
}
