//! 全排列 II


use std::collections::HashSet;

use leetcode::unorder;

pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &mut Vec<i32>, first: usize, ans: &mut Vec<Vec<i32>>, len: usize) {
        if first == len {
            ans.push(nums.to_vec());
        }
        let mut set = HashSet::new();
        for i in first..len {
            if set.contains(&nums[i]) { continue; }
            nums.swap(i, first);
            backtrack(nums, first + 1, ans, len);
            nums.swap(i, first);
            set.insert(nums[i]);
        }
    }
    let len = nums.len();
    let mut ans = vec![];
    backtrack(&mut nums, 0, &mut ans, len);
    ans
}

pub fn permute_unique_best(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut ans = vec![nums];

    for position in 0..len - 1 {
        for i in 0..ans.len() {
            ans[i][position..].sort_unstable();
            for j in position + 1..len {
                let mut tmp = ans[i].clone();
                if tmp[j] == tmp[j - 1] { continue; }
                tmp.swap(position, j);
                ans.push(tmp);
            }
        }
    }
    ans
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(unorder(func(vec![1, 1, 2])), unorder(vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]));
        assert_eq!(unorder(func(vec![1, 2, 3])), unorder(vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]]));
    }
    test(permute_unique);
    test(permute_unique_best);
}
