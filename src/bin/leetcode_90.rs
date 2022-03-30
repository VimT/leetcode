//! 子集 II


use leetcode::unorder;

pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut ans = vec![vec![]];
    let mut last_len = 1;
    for i in 0..nums.len() {
        let num = nums[i];
        let iter = if i > 0 && nums[i] == nums[i - 1] {
            ans[ans.len() - last_len..].to_vec()
        } else { ans.clone() };
        last_len = iter.len();
        for mut curr in iter {
            curr.push(num);
            ans.push(curr);
        }
    }
    ans
}


/// 回溯
pub fn subsets_with_dup_backtrack(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrace(nums: &Vec<i32>, first: usize, current: &mut Vec<i32>, k: usize, n: usize, ans: &mut Vec<Vec<i32>>) {
        if current.len() == k {
            ans.push(current.clone());
        }
        for i in first..n {
            if i > first && nums[i] == nums[i - 1] { continue; }
            current.push(nums[i]);
            backtrace(nums, i + 1, current, k, n, ans);
            current.pop();
        }
    }
    let mut ans = vec![];
    let len = nums.len();
    for k in 0..=len {
        backtrace(&nums, 0, &mut vec![], k, len, &mut ans);
    }
    ans
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(unorder(func(vec![1, 2, 2])), unorder(vec![vec![], vec![1], vec![1, 2], vec![1, 2, 2], vec![2], vec![2, 2]]));
        assert_eq!(unorder(func(vec![0])), unorder(vec![vec![], vec![0]]));
    }
    test(subsets_with_dup);
    test(subsets_with_dup_backtrack);
}
