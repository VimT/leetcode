//! 判断一个数的数字计数是否等于数位的值

pub fn digit_count(num: String) -> bool {
    let mut cnt = [0; 10];
    for &ch in num.as_bytes() {
        cnt[(ch - b'0') as usize] += 1;
    }
    for (i, &ch) in num.as_bytes().iter().enumerate() {
        if cnt[i] != (ch - b'0') {
            return false;
        }
    }
    true
}

fn main() {
    fn test(func: fn(num: String) -> bool) {
        assert_eq!(func(String::from("1210")), true);
        assert_eq!(func(String::from("030")), false);
    }
    test(digit_count);
}
