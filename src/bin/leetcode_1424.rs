//! 对角线遍历 II

use std::collections::BTreeMap;

pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let m = nums.len();
    let n = nums[0].len();
    let mut line_idx = BTreeMap::new();
    for i in 0..m {
        line_idx.insert(i, 0);
    }
    let mut result = Vec::with_capacity((m * n).min(1e5 as usize + 1));
    let mut s = 0;
    while !line_idx.is_empty() {
        let i = s.min(m - 1);
        let mut remove = vec![];
        for (&r, c) in line_idx.range_mut(..=i).rev() {
            result.push(nums[r][*c]);
            *c += 1;
            if *c >= nums[r].len() {
                remove.push(r);
            }
        }
        for x in remove {
            line_idx.remove(&x);
        }
        s += 1;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![1, 4, 2, 7, 5, 3, 8, 6, 9]);
        assert_eq!(func(vec![vec![1, 2, 3, 4, 5], vec![6, 7], vec![8], vec![9, 10, 11], vec![12, 13, 14, 15, 16]]), vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]);
    }
    test(find_diagonal_order);
}
