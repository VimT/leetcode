//! 栅栏涂色

/// 考虑第i个栅栏，如果第i个与i-1个颜色相同，则有 dp[i-2] * (k-1) 种涂法，如果第i个与第i-1个颜色不同，则有dp[i-1] * (k-1) 种涂法
pub fn num_ways(n: i32, k: i32) -> i32 {
    if n == 0 { return 0; } else if n == 1 { return k; } else if n == 2 { return k * k; }
    let mut p2 = k;
    let mut p1 = k * k;
    for _ in 2..n {
        let tmp = (k - 1) * (p1 + p2);
        p2 = p1;
        p1 = tmp;
    }
    p1
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> i32) {
        assert_eq!(func(3, 2), 6);
        assert_eq!(func(1, 1), 1);
        assert_eq!(func(7, 2), 42);
    }
    test(num_ways);
}
