//! 打家劫舍 II

/// dp[n] = max(dp[n-1], dp[n-2] + nums[n])
pub fn rob(nums: Vec<i32>) -> i32 {
    fn my_rob(nums: &[i32]) -> i32 {
        let mut cur = 0;
        let mut pre = 0;
        for i in nums {
            let tmp = cur;
            cur = cur.max(pre + i);
            pre = tmp;
        }
        cur
    }
    return if nums.len() > 1 {
        my_rob(&nums[..nums.len() - 1]).max(my_rob(&nums[1..]))
    } else {
        nums[0]
    };
}


fn main() {
    assert_eq!(rob(vec![2, 3, 2]), 3);
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![1, 2, 3]), 3);
}
