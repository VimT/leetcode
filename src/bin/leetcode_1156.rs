//! 单字符重复子串的最大长度

/// 滑动窗口，暴力模拟。。可以优化一下找左边或右边第一个字符的逻辑到O(1)
pub fn max_rep_opt1(text: String) -> i32 {
    let s = text.as_bytes();
    let mut i = 0;
    let len = s.len();
    let mut result = 0;
    while i < len {
        let ch = s[i];
        let mut end = i + 1;
        while end < len && s[end] == ch {
            end += 1;
        }
        if end == len {
            for j in 0..i {
                if s[j] == ch {
                    result = result.max(end - i + 1);
                    break;
                }
            }
            result = result.max(end - i);
            break;
        }
        let mid = end;
        let mut found = len;
        for j in 0..i {
            if s[j] == ch {
                found = j;
                break;
            }
        }
        if found == len {
            for j in (end..len).rev() {
                if s[j] == ch {
                    found = j;
                    break;
                }
            }
        }
        if found != len {
            end += 1;
            while end < if found > i { found } else { len } && s[end] == ch {
                end += 1;
            }
        }
        result = result.max(end - i);
        i = mid;
    }
    result as i32
}

fn main() {
    fn test(func: fn(text: String) -> i32) {
        assert_eq!(func(String::from("ababa")), 3);
        assert_eq!(func(String::from("aaabaaa")), 6);
        assert_eq!(func(String::from("aaaaa")), 5);
        assert_eq!(func(String::from("abcdef")), 1);
        assert_eq!(func(String::from("aabaaabaaaba")), 7);
        assert_eq!(func(String::from("acbaaa")), 4);
    }
    test(max_rep_opt1);
}
