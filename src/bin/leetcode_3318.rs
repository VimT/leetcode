//! 计算子数组的 x-sum I

use std::collections::HashMap;

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let len = nums.len();
    let (k, x) = (k as usize, x as usize);

    (0..=len - k).map(|l| {
        let nums = &nums[l..l + k];
        let mut cnts: HashMap<i32, i32> = Default::default();
        for &num in nums { *cnts.entry(num).or_default() += 1; }
        let mut cnts: Vec<(i32, i32)> = cnts.into_iter().map(|(k, v)| (-v, -k)).collect();
        cnts.sort_unstable();
        cnts[..x.min(cnts.len())].iter().map(|(a, b)| a * b).sum()
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32>) {
        assert_eq!(func(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2), vec![6, 10, 12]);
        assert_eq!(func(vec![3, 8, 7, 8, 7, 5], 2, 2), vec![11, 15, 15, 15, 12]);
    }
    test(find_x_sum);
}
