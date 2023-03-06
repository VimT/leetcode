//! 二进制数转字符串

pub fn print_bin(mut num: f64) -> String {
    let mut result = vec![b'0', b'.'];
    while result.len() <= 32 && num != 0.0 {
        num *= 2.;
        let c = num as u8;
        result.push(c + b'0');
        num -= c as f64;
    }
    if result.len() <= 32 {
        unsafe { String::from_utf8_unchecked(result) }
    } else {
        String::from("ERROR")
    }
}

fn main() {
    fn test(func: fn(num: f64) -> String) {
        assert_eq!(func(0.625), "0.101");
        assert_eq!(func(0.1), "ERROR");
        assert_eq!(func(0.573628), "ERROR");
    }
    test(print_bin);
}
