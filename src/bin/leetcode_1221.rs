//! 分割平衡字符串

pub fn balanced_string_split(s: String) -> i32 {
    let mut d = 0;
    let mut result = 0;
    for &ch in s.as_bytes() {
        if ch == b'L' { d += 1; } else { d -= 1; }
        if d == 0 { result += 1; }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("RLRRLLRLRL")), 4);
        assert_eq!(func(String::from("RLLLLRRRLR")), 3);
        assert_eq!(func(String::from("LLLLRRRR")), 1);
    }
    test(balanced_string_split);
}
