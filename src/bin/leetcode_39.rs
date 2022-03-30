//! 组合总和

use leetcode::unorder;

/// 分治
pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn inner(nums: &Vec<i32>, idx: usize, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        if target < 0 { return ans; }
        if target == 0 { return vec![vec![]]; }
        for i in idx..nums.len() {
            for mut sub in inner(nums, i, target - nums[i]) {
                sub.push(nums[i]);
                ans.push(sub);
            }
        }
        ans
    }
    candidates.sort_unstable();
    candidates.reverse();
    return inner(&candidates, 0, target);
}


/// 回溯
pub fn combination_sum_backtrace(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    pub fn backtrack(seq: &Vec<i32>, target: i32, start: i32, vec: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if target < 0 { return; }
        if target == 0 && vec.len() != 0 {
            res.push(vec.clone());
            return;
        }

        for &num in seq {
            if num < start { continue; }
            vec.push(num);
            backtrack(seq, target - num, num, vec, res);
            vec.pop();
        }
    }

    let mut res = Vec::new();
    candidates.sort_unstable();
    let mut vec = Vec::new();
    backtrack(&candidates, target, 0, &mut vec, &mut res);
    return res;
}

fn main() {
    fn test(func: fn(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>) {
        assert_eq!(unorder(func(vec![2, 3, 6, 7], 7)), unorder(vec![vec![2, 2, 3], vec![7]]));
        assert_eq!(unorder(func(vec![2, 3, 5], 8)), unorder(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]));
        assert_eq!(unorder(func(vec![2], 1)).is_empty(), true);
    }
    test(combination_sum);
    test(combination_sum_backtrace);
}
