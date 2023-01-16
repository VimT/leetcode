//! 范围内最接近的两个质数

use std::sync::Once;

static mut PRIME: Vec<i32> = vec![];
static ONCE: Once = Once::new();

fn cal_prime(n: i32) -> Vec<i32> {
    let mut result = vec![2, 3, 5, 7, 11, 13];
    for i in 17..=n {
        let mut ok = true;
        for &num in &result {
            if num * num > i {
                break;
            }
            if i % num == 0 {
                ok = false;
                break;
            }
        }
        if ok { result.push(i); }
    }
    result
}

/// 欧拉筛
fn cal_prime2(n: i32) -> Vec<i32> {
    let mut result = vec![];
    let n = n as usize;
    let mut is_prime = vec![true; n + 1];
    for i in 2..=n {
        if is_prime[i] { result.push(i as i32); }
        for &p in &result {
            let p = p as usize;
            if i * p >= n { break; }
            is_prime[i * p] = false;
            if i % p == 0 { break; }
        }
    }
    result
}

pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    unsafe {
        ONCE.call_once(|| { PRIME = cal_prime2(1e6 as i32); });
        let l = PRIME.binary_search(&left).unwrap_or_else(|x| x);
        let r = PRIME.binary_search(&right).map(|x| x + 1).unwrap_or_else(|x| x);
        let mut result = vec![-1, -1];
        let mut min = i32::MAX;
        for i in l + 1..r {
            if PRIME[i] - PRIME[i - 1] < min {
                result = vec![PRIME[i - 1], PRIME[i]];
                min = PRIME[i] - PRIME[i - 1];
            }
        }
        result
    }
}

fn main() {
    fn test(func: fn(left: i32, right: i32) -> Vec<i32>) {
        assert_eq!(func(10, 19), vec![11, 13]);
        assert_eq!(func(4, 6), vec![-1, -1]);
        assert_eq!(func(1, 1), vec![-1, -1]);
    }
    test(closest_primes);
}
