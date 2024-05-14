//! 替换字符串中的问号使分数最小

pub fn minimize_string_value(mut origin: String) -> String {
    let s = unsafe { origin.as_bytes_mut() };
    let len = s.len();
    let mut cnt = [0; 26];
    for i in 0..len {
        if s[i] != b'?' {
            cnt[(s[i] - b'a') as usize] += 1;
        }
    }
    let mut fill = vec![];
    for i in 0..len {
        if s[i] == b'?' {
            let ch = (0..26).min_by_key(|&x| (cnt[x], x)).unwrap() as u8 + b'a';
            fill.push(ch);
            cnt[(ch - b'a') as usize] += 1;
        }
    }
    fill.sort_unstable();
    fill.reverse();
    for i in 0..len {
        if s[i] == b'?' {
            s[i] = fill.pop().unwrap();
        }
    }

    origin
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("bc?a")), String::from("bcda"));
        assert_eq!(func(String::from("abcdefghijklmnopqrstuvwxy??")), String::from("abcdefghijklmnopqrstuvwxyaz"));
        assert_eq!(func(String::from("???")), String::from("abc"));
        assert_eq!(func(String::from("a?a?")), String::from("abac"));
    }
    test(minimize_string_value);
}
