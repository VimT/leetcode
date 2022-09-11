//! 丑数 III

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { return b; }
    return gcd(b % a, a);
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

/// 容斥原理，二分查找
pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
    let n = n as i64;
    let la = a as i64;
    let lb = b as i64;
    let lc = c as i64;
    let lab = lcm(la, lb);
    let lac = lcm(la, lc);
    let lbc = lcm(lb, lc);
    let labc = lcm(lab, lc);
    let maxabc = a.max(b).max(c) as i64;
    let count_less_equal = |x| {
        x / la + x / lb + x / lc - x / lab - x / lac - x / lbc + x / labc
    };
    let m = count_less_equal(labc);
    let q = n / m;
    let r = n % m;
    let mut hi = labc.min(r * maxabc);
    let mut lo = 0;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if count_less_equal(mid) < r {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    (lo + q * labc) as i32
}

fn main() {
    fn test(func: fn(n: i32, a: i32, b: i32, c: i32) -> i32) {
        assert_eq!(func(3, 2, 3, 5), 4);
        assert_eq!(func(4, 2, 3, 4), 6);
        assert_eq!(func(5, 2, 11, 13), 10);
    }
    test(nth_ugly_number);
}
