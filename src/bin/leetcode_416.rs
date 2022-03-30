//! 分割等和子集

/// 0-1 package
pub fn can_partition(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len < 2 { return false; }
    let mut sum = 0;
    let mut max = 0;
    for i in &nums {
        sum += *i;
        max = max.max(*i);
    }
    let target = sum / 2;
    if max > target || sum % 2 == 1 {
        return false;
    }
    let target = target as usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for i in 0..len {
        let num = nums[i] as usize;
        for j in (num..=target).rev() {
            dp[j] |= dp[j - num];
        }
    }
    dp[target]
}

fn main() {
    assert_eq!(can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(can_partition(vec![1, 2, 3, 5]), false);
}
