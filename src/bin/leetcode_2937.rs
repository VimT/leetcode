//! 使三个字符串相等

pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
    let mut same = 0;
    for ((a, b), c) in s1.as_bytes().iter().zip(s2.as_bytes()).zip(s3.as_bytes()) {
        if a != b || b != c || a != c { break; }
        same += 1;
    }
    if same == 0 { return -1; }
    return (s1.len() + s2.len() + s3.len() - 3 * same) as i32;
}

fn main() {
    fn test(func: fn(s1: String, s2: String, s3: String) -> i32) {
        assert_eq!(func(String::from("abc"), String::from("abb"), String::from("ab")), 2);
        assert_eq!(func(String::from("dac"), String::from("bac"), String::from("cac")), -1);
    }
    test(find_minimum_operations);
}
