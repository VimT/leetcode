//! 质数减法运算

use std::cmp::Ordering;
use std::sync::Once;

pub fn cal_prime(n: usize) -> Vec<usize> {
    let mut result = vec![0];
    let mut is_prime = vec![true; n + 1];
    for i in 2..=n {
        if is_prime[i] { result.push(i); }
        for &p in &result[1..] {
            if i * p >= n { break; }
            is_prime[i * p] = false;
            if i % p == 0 { break; }
        }
    }
    result
}


static mut PRIMES: Vec<usize> = vec![];
static mut ONCE: Once = Once::new();

unsafe fn init() {
    ONCE.call_once(|| {
        PRIMES = cal_prime(1001);
    });
}

pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
    unsafe {
        init();
        let mut pre = 0;
        for num in nums {
            if num <= pre {return false;}
            let j = PRIMES.binary_search_by(|x|x.cmp(&((num - pre) as usize)).then(Ordering::Greater)).unwrap_err() - 1;
            pre = num - PRIMES[j] as i32;
        }
        true
    }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![998, 2]), true);
        assert_eq!(func(vec![4, 9, 6, 10]), true);
        assert_eq!(func(vec![6, 8, 11, 12]), true);
        assert_eq!(func(vec![5, 8, 3]), false);
    }
    test(prime_sub_operation);
}
