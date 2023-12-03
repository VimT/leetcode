//! 爬楼梯

use leetcode::algorithm::matrix_power;

pub fn climb_stairs(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    for _ in 0..n - 1 {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

pub fn climb_stairs2(n: i32) -> i32 {
    matrix_power(vec![vec![1, 1], vec![1, 0]], n as i64, i64::MAX)[0][0] as i32
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(2), 2);
        assert_eq!(func(3), 3);
    }
    test(climb_stairs);
    test(climb_stairs2);
}
