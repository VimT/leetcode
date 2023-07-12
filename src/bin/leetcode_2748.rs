//! 美丽下标对的数目

use leetcode::gcd::gcd;

pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
    let mut cnt = [0; 10];
    let mut result = 0;
    for mut num in nums {
        for j in 1..=9 {
            if gcd(num % 10, j) == 1 {
                result += cnt[j as usize];
            }
        }
        while num >= 10 { num /= 10; }
        cnt[num as usize] += 1;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 5, 1, 4]), 5);
        assert_eq!(func(vec![11, 21, 12]), 2);
    }
    test(count_beautiful_pairs);
}
