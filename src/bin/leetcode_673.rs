//! 最长递增子序列的个数

pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 { return 1; }
    let mut dp = vec![1; len];
    let mut count = vec![1; len];
    let mut max = 0;
    for i in 1..len {
        for j in 0..i {
            if nums[j] < nums[i] {
                if dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    count[i] = count[j];
                } else if dp[j] + 1 == dp[i] {
                    count[i] += count[j];
                }
            }
        }
        max = max.max(dp[i]);
    }
    let mut ans = 0;
    for i in 0..len {
        if dp[i] == max {
            ans += count[i];
        }
    }
    ans
}


fn main() {
    assert_eq!(find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    assert_eq!(find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
}
