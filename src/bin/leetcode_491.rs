//! 递增子序列


use leetcode::unorder;

pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(nums: &Vec<i32>, cur: usize, last: i32, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if cur == nums.len() {
            if tmp.len() >= 2 {
                ans.push(tmp.clone());
            }
            return;
        }
        if nums[cur] >= last {
            tmp.push(nums[cur]);
            dfs(nums, cur + 1, nums[cur], tmp, ans);
            tmp.pop();
        }
        if nums[cur] != last {
            dfs(nums, cur + 1, last, tmp, ans);
        }
    }
    let mut ans = vec![];
    dfs(&nums, 0, i32::MIN, &mut vec![], &mut ans);
    ans
}

/// find from right
pub fn find_subsequences_dp(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![]; len];
    for i in 0..len {
        dp[i].push(vec![nums[i]]);
    }
    for i in (0..len).rev() {
        for j in i + 1..len {
            if nums[j] >= nums[i] {
                for k in 0..dp[j].len() {
                    let mut tmp = vec![nums[i]];
                    tmp.extend_from_slice(&dp[j][k]);
                    dp[i].push(tmp);
                }
                // remove duplicate
                if nums[j] == nums[i] {
                    dp[j].clear();
                }
            }
        }
    }
    dp.into_iter().flatten().filter(|x| x.len() > 1).collect()
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(unorder(func(vec![4, 6, 7, 7])), vec![vec![4, 6], vec![4, 6, 7], vec![4, 6, 7, 7], vec![4, 7], vec![4, 7, 7], vec![6, 7], vec![6, 7, 7], vec![7, 7]]);
        assert_eq!(unorder(func(vec![4, 4, 3, 2, 1])), vec![vec![4, 4]]);
    }
    test(find_subsequences);
    test(find_subsequences_dp);
}
