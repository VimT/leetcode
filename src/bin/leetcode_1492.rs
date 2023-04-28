//! n 的第 k 个因子

pub fn kth_factor(n: i32, k: i32) -> i32 {
    let mut cnt = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            cnt += 1;
            if cnt == k { return i; }
        }
        i += 1;
    }
    i -= 1;
    if i * i == n { i -= 1; }
    while i > 0 {
        if n % i == 0 {
            cnt += 1;
            if cnt == k { return n / i; }
        }
        i -= 1;
    }
    -1
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> i32) {
        assert_eq!(func(12, 3), 3);
        assert_eq!(func(7, 2), 7);
        assert_eq!(func(4, 4), -1);
    }
    test(kth_factor);
}
