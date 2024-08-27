//! 统计近似相等数对 I

use std::collections::{HashMap, HashSet};

pub fn count_pairs(nums: Vec<i32>) -> i32 {
    fn to_number(digits: &[i32]) -> i32 {
        digits.iter().fold(0, |num, &digit| num * 10 + digit)
    }
    fn to_digits(mut num: i32, len: usize) -> Vec<i32> {
        let mut digits = Vec::with_capacity(len);
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.resize(len, 0);
        digits.reverse();
        digits
    }

    let mut result = 0;
    let mut seen = HashMap::new();
    let max_len = nums.iter().map(|&num| num.to_string().len()).max().unwrap();

    for &origin_num in &nums {
        let mut digital = to_digits(origin_num, max_len);
        let mut all_possible = HashSet::new();
        all_possible.insert(origin_num);
        let len = digital.len();
        for i in 0..len {
            for j in i + 1..len {
                digital.swap(i, j);
                let after_num = to_number(&digital);
                all_possible.insert(after_num);
                digital.swap(i, j);
            }
        }
        for &num in &all_possible {
            if let Some(&count) = seen.get(&num) { result += count; }
        }
        *seen.entry(origin_num).or_insert(0) += 1;
    }

    result
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![5, 12, 8, 5, 5, 1, 20, 3, 10, 10, 5, 5, 5, 5, 1]), 27);
        assert_eq!(func(vec![1, 1, 1, 1, 1]), 10);
        assert_eq!(func(vec![3, 12, 30, 17, 21]), 2);
        assert_eq!(func(vec![123, 231]), 0);
    }
    test(count_pairs);
}
