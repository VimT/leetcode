//! 划分数组得到最小的值之和

use std::collections::HashMap;

/// 一段数组的 and / or / gcd / lcm 数量是 logn 个
pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
    fn dfs(d @ (nums, and_values): &(Vec<i32>, Vec<i32>), i: usize, j: usize, and: i32, cache: &mut Vec<Vec<HashMap<i32, i32>>>) -> i32 {
        if nums.len() - i < and_values.len() - j { return i32::MAX / 2; } // 剩余元素不足
        if j == and_values.len() { return if i == nums.len() { 0 } else { i32::MAX / 2 }; }
        if i == nums.len() { return i32::MAX / 2; }
        if let Some(&x) = cache[i][j].get(&and) { return x; }
        let nand = and & nums[i];
        if nand < and_values[j] { return i32::MAX / 2; }
        let mut result = dfs(d, i + 1, j, nand, cache);
        if nand == and_values[j] {
            result = result.min(dfs(d, i + 1, j + 1, i32::MAX, cache) + nums[i]);
        }
        cache[i][j].insert(and, result);
        result
    }
    let mut cache = vec![vec![HashMap::new(); and_values.len()]; nums.len()];
    let x = dfs(&(nums, and_values), 0, 0, i32::MAX, &mut cache);
    if x >= i32::MAX / 2 { -1 } else { x }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, and_values: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2]), 12);
        assert_eq!(func(vec![4, 8, 9], vec![0]), 9);
        assert_eq!(func(vec![2, 3, 5, 7, 7, 7, 5], vec![0, 7, 5]), 17);
        assert_eq!(func(vec![1, 2, 3, 4], vec![2]), -1);
    }
    test(minimum_value_sum);
}
