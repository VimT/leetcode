//! 公因子的数目

pub fn common_factors(a: i32, b: i32) -> i32 {
    let mut result = 0;
    for i in 1..=a.min(b) {
        if a % i == 0 && b % i == 0 {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(a: i32, b: i32) -> i32) {
        assert_eq!(func(12, 6), 4);
        assert_eq!(func(25, 30), 2);
    }
    test(common_factors);
}
