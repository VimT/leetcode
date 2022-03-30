//! 等差数列划分 II - 子序列

use std::collections::HashMap;

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); len];
    let mut result = 0;
    for i in 1..len {
        for j in 0..i {
            let diff = nums[i] as i64 - nums[j] as i64;
            let cnt = *dp[j].get(&diff).unwrap_or(&0);
            result += cnt;
            *dp[i].entry(diff).or_default() += cnt + 1;
        }
    }
    result
}

fn main() {
    assert_eq!(number_of_arithmetic_slices(vec![0, 2000000000, -294967296]), 0);
    assert_eq!(number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]), 7);
    assert_eq!(number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]), 16);
}
