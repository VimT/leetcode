//! 统计桌面上的不同数字

pub fn distinct_integers(n: i32) -> i32 {
    if n == 1 { 1 } else { n - 1 }
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(5), 4);
        assert_eq!(func(3), 2);
    }
    test(distinct_integers);
}
