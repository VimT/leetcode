//! K 次增加后的最大乘积

use std::collections::BinaryHeap;

/// 贪心，堆
pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut heap: BinaryHeap<i32> = nums.into_iter().map(|x| -x).collect();
    for _ in 0..k {
        let num = heap.pop().unwrap();
        heap.push(num - 1);
    }
    let mut result = 1;
    while !heap.is_empty() {
        result = (result * -heap.pop().unwrap() as i64) % MOD;
    }
    result as i32
}

/// 每次+1效率太低，想办法一次多加点
pub fn maximum_product_optimise(mut nums: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    nums.sort_unstable();
    let mut cur_sum = 0;
    for i in 0..nums.len() {
        cur_sum += nums[i] as i64;
        let next = if i + 1 < nums.len() { nums[i + 1] } else { i32::MAX };
        if next as i64 * (i + 1) as i64 - cur_sum > k as i64 {
            // 所有数字对齐到 i，然后再均匀撒
            let mut all_same = nums[i];
            let same_cnt = i as i32 + 1;
            let add = k - (all_same as i32 * same_cnt - cur_sum as i32);
            all_same += add / same_cnt;
            let more1_cnt = add % same_cnt;
            let mut result = 1;
            for _ in 0..(same_cnt - more1_cnt) {
                result = (result * all_same as i64) % MOD;
            }
            for _ in 0..more1_cnt {
                result = (result * (all_same as i64 + 1)) % MOD;
            }
            for &num in &nums[i + 1..] {
                result = (result * num as i64) % MOD;
            }
            return result as i32;
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![7, 10, 6, 1, 8], 5), 20160);
        assert_eq!(func(vec![0, 4], 5), 20);
        assert_eq!(func(vec![6, 3, 3, 2], 2), 216);
    }
    test(maximum_product);
    test(maximum_product_optimise);
}
