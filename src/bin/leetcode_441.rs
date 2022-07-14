//! 排列硬币

pub fn arrange_coins(n: i32) -> i32 {
    (((8. * n as f64 + 1.).sqrt() - 1.) / 2.) as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(5), 2);
        assert_eq!(func(8), 3);
    }
    test(arrange_coins);
}
