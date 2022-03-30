//! 超级次方

/// 2^(137) == ((2^1)^10 * 2^3)^10 * 2^7;
pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
    const YU: i32 = 1337;
    fn pow(mut a: i32, b: i32) -> i32 {
        a %= YU;
        let mut ret = 1;
        for _ in 0..b {
            ret *= a;
            ret %= YU;
        }
        ret
    }
    if a == 1 { return 1; }
    let mut sum = 1;
    for i in b {
        sum = pow(sum, 10);
        sum *= pow(a, i);
        sum %= YU;
    }
    sum
}

fn main() {
    assert_eq!(super_pow(2, vec![3]), 8);
    assert_eq!(super_pow(2, vec![1, 0]), 1024);
    assert_eq!(super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
    assert_eq!(super_pow(2147483647, vec![2, 0, 0]), 1198);
}