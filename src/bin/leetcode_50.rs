//! Pow(x, n)

pub fn my_pow(mut x: f64, n: i32) -> f64 {
    let mut p = n as i64;
    if p < 0 {
        x = 1f64 / x;
        p = -p;
    }
    let mut ans = 1f64;
    let mut current_product = x;
    while p != 0 {
        if p % 2 == 1 {
            ans *= current_product;
        }
        current_product *= current_product;
        p /= 2;
    }
    ans
}

pub fn my_pow_re(mut x: f64, mut n: i32) -> f64 {
    fn fastpow(x: f64, n: i32) -> f64 {
        if n == 0 { return 1f64; }
        let ans = my_pow_re(x, n / 2);
        return if n % 2 == 0 {
            ans * ans
        } else {
            ans * ans * x
        };
    }
    if n < 0 {
        x = 1f64 / x;
        n = -n;
    }
    fastpow(x, n)
}

fn main() {
    fn test(func: fn(x: f64, n: i32) -> f64) {
        assert_eq!(func(2.00000, 10) - 1024.00000 < 1.0e-5, true);
        assert_eq!(func(2.10000, 3) - 9.26100 < 1.0e-5, true);
        assert_eq!(func(2.00000, -2) - 0.25000 < 1.0e-5, true);
    }
    test(my_pow);
    test(my_pow_re);
}
