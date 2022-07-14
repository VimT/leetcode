//! 找到一个数字的 K 美丽值

pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    let mut result = 0;
    for sub in num.to_string().as_bytes().windows(k as usize) {
        unsafe {
            let sub_num = std::str::from_utf8_unchecked(sub).parse::<i32>().unwrap();
            if sub_num != 0 && num % sub_num == 0 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(num: i32, k: i32) -> i32) {
        assert_eq!(func(240, 2), 2);
        assert_eq!(func(430043, 2), 2);
    }
    test(divisor_substrings);
}
