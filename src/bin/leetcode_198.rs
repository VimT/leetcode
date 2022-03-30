//! 打家劫舍


pub fn rob(nums: Vec<i32>) -> i32 {
    fn inner(nums: &Vec<i32>, current: usize) -> i32 {
        if current >= nums.len() {
            return 0;
        }
        return (nums.get(current).unwrap_or(&0) + inner(nums, current + 2)).max(nums.get(current + 1).unwrap_or(&0) + inner(nums, current + 3));
    }
    return inner(&nums, 0);
}

/// dp[n] = max(dp[n-2] + n[i], dp[n-1])
pub fn rob_dp(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0; }
    let mut prev = 0;
    let mut current = 0;
    for i in nums {
        let tmp = current;
        current = (prev + i).max(current);
        prev = tmp;
    }
    return current;
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 1]), 4);
        assert_eq!(func(vec![2, 7, 9, 3, 1]), 12);
    }
    test(rob);
    test(rob_dp);
}
