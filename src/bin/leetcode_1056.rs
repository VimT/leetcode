//! 易混淆数

pub fn confusing_number(n: i32) -> bool {
    let mut new_num = 0;
    let mut num = n;
    while num > 0 {
        let wei = num % 10;
        new_num = new_num * 10 + match wei {
            0 => 0,
            1 => 1,
            6 => 9,
            8 => 8,
            9 => 6,
            _ => { return false; }
        };
        num /= 10;
    }
    return new_num != n;
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(6), true);
        assert_eq!(func(89), true);
        assert_eq!(func(11), false);
    }
    test(confusing_number);
}
