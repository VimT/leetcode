//! 一个小组的最大实力值

pub fn max_strength(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut result = nums.iter().max().copied().unwrap() as i64;
    for i in 1..1 << len {
        let mut score = 1;
        for j in 0..len {
            if i >> j & 1 == 1 {
                score *= nums[j] as i64;
            }
        }
        result = result.max(score);
    }
    result
}

pub fn max_strength2(nums: Vec<i32>) -> i64 {
    fn dfs(nums: &Vec<i32>, i: usize, cur: i64, result: &mut i64, select: bool) {
        if i == nums.len() {
            if select && cur > *result {
                *result = cur;
            }
            return;
        }
        dfs(nums, i + 1, cur, result, select);
        dfs(nums, i + 1, cur * nums[i] as i64, result, true);
    }
    let mut result = i64::MIN;
    dfs(&nums, 0, 1, &mut result, false);
    result
}


/// 动态规划，O(n)
pub fn max_strength3(nums: Vec<i32>) -> i64 {
    let mut mn = nums[0] as i64;
    let mut mx = nums[0] as i64;
    for &num in &nums[1..] {
        let tmp = mx;
        let num = num as i64;
        mx = mx.max(num).max(mx * num).max(mn * num);
        mn = mn.min(num).min(tmp * num).min(mn * num);
    }
    mx
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![3, -1, -5, 2, 5, -9]), 1350);
        assert_eq!(func(vec![-4, -5, -4]), 20);
    }
    test(max_strength);
    test(max_strength2);
    test(max_strength3);
}
