//! 最长的斐波那契子序列的长度


use std::collections::HashMap;

pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut m = HashMap::new();
    for i in 0..len {
        m.insert(arr[i], i);
    }
    let mut dp = vec![vec![2; len]; len];
    let mut result = 0;
    for i in 2..len {
        for j in (1..i).rev() {
            if arr[i] - arr[j] < arr[j] {
                if let Some(&k) = m.get(&(arr[i] - arr[j])) {
                    dp[i][j] = dp[j][k] + 1;
                    result = result.max(dp[i][j]);
                }
            }
        }
    }
    result
}

fn main() {
    assert_eq!(len_longest_fib_subseq(vec![3, 4, 8, 11, 12, 15, 16, 20, 21, 22, 24, 26, 27, 28, 30, 39, 42, 43, 44, 45, 47, 50, 51, 53, 56, 58, 64, 72, 75, 78, 80, 81, 86, 103, 105, 107, 108, 116, 120, 123, 129, 130, 142, 156, 163, 171, 172, 188, 195, 204, 210, 228, 259, 278, 280, 304, 315, 327, 340, 370, 415, 449, 452, 550, 598, 674, 727, 732, 890, 1089, 1176, 1184, 1440, 1763, 1903, 1916, 2330, 2852, 3079, 3100, 3770, 4615, 4982, 5016, 6100, 7467, 8061, 8116, 9870, 12082, 13043, 13132, 15970, 19549, 21104, 21248, 31631, 34147, 34380, 51180, 55251, 55628, 90008, 145636]), 19);
    assert_eq!(len_longest_fib_subseq(vec![2, 4, 5, 6, 7, 8, 11, 13, 14, 15, 21, 22, 34]), 5);
    assert_eq!(len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]), 5);
    assert_eq!(len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]), 3);
}
