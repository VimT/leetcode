//! 四因数

use std::sync::Once;
use leetcode::algorithm::cal_prime;

const LEN: usize = 1e5 as usize + 2;
static mut DATA: [i32; LEN] = [0; LEN];
static ONCE: Once = Once::new();

unsafe fn init() {
    ONCE.call_once(|| {
        let prime = cal_prime(LEN);
        for (i, &a) in prime.iter().enumerate() {
            if a < 46 { DATA[a * a * a] = (1 + a + a * a + a * a * a) as i32; }
            for &b in &prime[i + 1..] {
                if a * b >= LEN { break; }
                DATA[a * b] = (1 + a + b + a * b) as i32;
            }
        }
    })
}

pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    unsafe {
        init();
        let mut result = 0;
        for num in nums {
            result += DATA[num as usize]
        }
        result
    }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![7286, 18704, 70773, 8224, 91675]), 10932);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 45);
        assert_eq!(func(vec![21, 4, 7]), 32);
        assert_eq!(func(vec![21, 21]), 64);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 0);
    }
    test(sum_four_divisors);
}
