//! 带因子的二叉树

use std::collections::HashMap;

pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    arr.sort_unstable();
    let len = arr.len();
    let mut dp = vec![1i64; len];
    let mut map = HashMap::new();
    for i in 0..len {
        map.insert(arr[i], i);
    }
    for i in 0..len {
        for j in 0..i {
            if arr[i] % arr[j] == 0 {
                let right = arr[i] / arr[j];
                if let Some(&right_idx) = map.get(&right) {
                    dp[i] = (dp[i] + dp[j] * dp[right_idx]) % MOD;
                }
            }
        }
    }
    let mut result = 0;
    for i in 0..len {
        result = (result + dp[i]) % MOD;
    }
    result as i32
}

fn main() {
    assert_eq!(num_factored_binary_trees(vec![45, 42, 2, 18, 23, 1170, 12, 41, 40, 9, 47, 24, 33, 28, 10, 32, 29, 17, 46, 11, 759, 37, 6, 26, 21, 49, 31, 14, 19, 8, 13, 7, 27, 22, 3, 36, 34, 38, 39, 30, 43, 15, 4, 16, 35, 25, 20, 44, 5, 48]), 777);
    assert_eq!(num_factored_binary_trees(vec![2, 4]), 3);
    assert_eq!(num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
}
