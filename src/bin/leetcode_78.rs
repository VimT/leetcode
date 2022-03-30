//! 子集


use leetcode::unorder;

pub fn subsets_recursion(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![vec![]];
    for &num in &nums {
        for mut curr in ans.clone() {
            curr.push(num);
            ans.push(curr);
        }
    }
    ans
}

/// 回溯
pub fn subsets_backtrack(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrace(nums: &Vec<i32>, first: usize, current: &mut Vec<i32>, k: usize, n: usize, ans: &mut Vec<Vec<i32>>) {
        if current.len() == k {
            ans.push(current.clone());
        }
        for i in first..n {
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

/// 二进制排序
pub fn subsets_bit(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut ans = vec![];
    for i in 2_usize.pow(len as u32)..2_usize.pow((len + 1) as u32) {
        let bitmask = format!("{:b}", i)[1..].to_string().into_bytes();
        let mut curr = vec![];
        for j in 0..len {
            if bitmask[j] == b'1' {
                curr.push(nums[j]);
            }
        }
        ans.push(curr);
    }
    ans
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(unorder(func(vec![1, 2, 3])), unorder(vec![vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]]));
        assert_eq!(unorder(func(vec![0])), unorder(vec![vec![], vec![0]]));
    }
    test(subsets_backtrack);
    test(subsets_bit);
    test(subsets_recursion);
}
