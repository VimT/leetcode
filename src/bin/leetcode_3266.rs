//! K 次乘运算后的最终数组 II

use leetcode::algorithm::quick_pow;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 要先让所有值接近
pub fn get_final_state(nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
    const MOD: i64 = 1_000_000_007;
    if multiplier == 1 { return nums; }
    let multiplier = multiplier as i64;
    let len = nums.len();
    let mx = *nums.iter().max().unwrap() as i64;
    let mut q: BinaryHeap<_> = nums.into_iter().map(|x| x as i64).zip(0..).map(Reverse).collect();
    while k > 0 {
        if q.peek().unwrap().0.0 * multiplier > mx { break; }
        k -= 1;
        let Reverse((num, idx)) = q.pop().unwrap();
        q.push(Reverse((num * multiplier, idx)));
    }
    let c = (k as usize) / len;
    let mut r = (k as usize) % len;
    let e = quick_pow(multiplier, c as i64, MOD);
    let mut result = vec![0; len];
    while let Some(Reverse((num, idx))) = q.pop() {
        let mut tmp = num * e % MOD;
        if r > 0 {
            tmp = tmp * multiplier % MOD;
            r -= 1;
        }
        result[idx] = tmp as i32;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32>) {
        assert_eq!(func(vec![2, 1, 3, 5, 6], 5, 2), vec![8, 4, 6, 5, 6]);
        assert_eq!(func(vec![100000, 2000], 2, 1000000), vec![999999307, 999999993]);
    }
    test(get_final_state);
}
