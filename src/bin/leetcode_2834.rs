//! 找出美丽数组的最小和

pub fn minimum_possible_sum(n: i32, k: i32) -> i64 {
    let n = n as i64;
    let k = k as i64;
    let m = n.min(k / 2);
    (m * (m + 1) + (k * 2 + n - m - 1) * (n - m)) / 2
}

fn main() {
    fn test(func: fn(n: i32, target: i32) -> i64) {
        assert_eq!(func(2, 3), 4);
        assert_eq!(func(3, 3), 8);
        assert_eq!(func(1, 1), 1);
    }
    test(minimum_possible_sum);
}
