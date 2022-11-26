//! 子字符串的最优划分

pub fn partition_string(s: String) -> i32 {
    let s = s.as_bytes();
    let mut cur = [false; 26];
    let mut result = 0;
    for &ch in s {
        if cur[(ch - b'a') as usize] {
            cur.fill(false);
            result += 1;
        }
        cur[(ch - b'a') as usize] = true;
    }
    result + 1
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("abacaba")), 4);
        assert_eq!(func(String::from("ssssss")), 6);
    }
    test(partition_string);
}
