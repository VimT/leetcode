//! 使字符串平衡的最少删除次数

pub fn minimum_deletions(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut right_a = vec![0; len + 1];
    let mut cur = 0;
    for i in (0..len).rev() {
        if s[i] == b'a' {
            cur += 1;
        }
        right_a[i] = cur;
    }
    cur = 0;
    let mut result = right_a[0];
    for i in 0..len {
        if s[i] == b'b' {
            cur += 1;
        }
        result = result.min(cur + right_a[i + 1]);
    }
    result
}

pub fn minimum_deletions2(s: String) -> i32 {
    let len = s.len();
    let mut b_count = s.chars().filter(|&x| x == 'b').count();
    let mut a_count = 0;

    s.bytes().fold(b_count.min(len - b_count), |s, x| {
        if x == b'a' {
            a_count += 1;
        } else {
            b_count -= 1;
        }
        s.min(len - a_count - b_count)
    }) as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("b")), 0);
        assert_eq!(func(String::from("aababbab")), 2);
        assert_eq!(func(String::from("bbaaaaabb")), 2);
    }
    test(minimum_deletions);
    test(minimum_deletions2);
}
