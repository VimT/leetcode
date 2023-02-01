//! 具有给定数值的最小字符串

pub fn get_smallest_string(n: i32, mut k: i32) -> String {
    let mut result = vec![0; n as usize];
    for i in (0..n).rev() {
        let choose = (k - i).min(26);
        result[i as usize] = choose as u8 + b'a' - 1;
        k -= choose;
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> String) {
        assert_eq!(func(3, 27), String::from("aay"));
        assert_eq!(func(5, 73), String::from("aaszz"));
    }
    test(get_smallest_string);
}
