//! 统计近似相等数对 II

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

                for k in 0..len {
                    for l in k + 1..len {
                        digital.swap(k, l);
                        let after_num2 = to_number(&digital);
                        all_possible.insert(after_num2);
                        digital.swap(k, l);
                    }
                }
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
        assert_eq!(func(vec![1023, 2310, 2130, 213]), 4);
        assert_eq!(func(vec![1, 10, 100]), 3);
    }
    test(count_pairs);
}