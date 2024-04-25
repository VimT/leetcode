//! 素数的最大距离


use leetcode::algorithm::is_prime;

pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    while !is_prime(nums[i]) {
        i += 1;
    }
    let mut j = nums.len();
    while !is_prime(nums[j - 1]) {
        j -= 1;
    }
    (j - 1 - i) as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![4, 2, 9, 5, 3]), 3);
        assert_eq!(func(vec![4, 8, 2, 8]), 0);
    }
    test(maximum_prime_difference);
}
