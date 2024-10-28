//! 字符串转换后的长度 II

use leetcode::algorithm::{matrix_mul, matrix_power};

pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
    let s = s.as_bytes();
    let mut cnt = vec![0; 26];
    for &ch in s {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut a = vec![vec![0; 26]; 26];
    for i in 0..26 {
        for k in 1..=nums[i] {
            a[(i + k as usize) % 26][i] = 1;
        }
    }
    const MOD: i64 = 1e9 as i64 + 7;
    let b = matrix_power(a, t as i64, MOD);
    let cnt = matrix_mul(&b, &cnt.into_iter().map(|x| vec![x]).collect(), MOD);
    let mut result = 0;
    for c in cnt {
        result = (result + c[0]) % MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String, t: i32, nums: Vec<i32>) -> i32) {
        assert_eq!(func(String::from("abcyy"), 2, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]), 7);
        assert_eq!(func(String::from("azbk"), 1, vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 8);
    }
    test(length_after_transformations);
}
