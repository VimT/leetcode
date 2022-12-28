//! 目标和


pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    let mut result = 0;
    let left = len / 2;
    let right = len - left;
    const NUM: usize = 20000;
    let mut map = vec![0; 2 * NUM];
    for i in 0..1 << left {
        let mut num = 0;
        for j in 0..left {
            if i >> j & 1 == 1 {
                num += nums[j];
            } else {
                num -= nums[j];
            }
        }
        map[(num + NUM as i32) as usize] += 1;
    }
    for i in 0..1 << right {
        let mut num = 0;
        for j in 0..right {
            if i >> j & 1 == 1 {
                num += nums[left + j];
            } else {
                num -= nums[left + j];
            }
        }
        result += map[(target - num + NUM as i32) as usize];
    }
    result
}

/// p - (sum - p) = target
/// p = (target + sum) / 2   p为nums中正数的和
/// 所以转换为0-1背包问题
pub fn find_target_sum_ways2(nums: Vec<i32>, target: i32) -> i32 {
    let limit = nums.iter().sum::<i32>() + target;
    if limit < 0 || limit & 1 == 1 {
        return 0;
    }
    let target = (limit / 2) as usize;
    let mut dp = vec![0; target + 1];
    dp[0] = 1;
    for num in nums {
        for j in (num as usize..=target).rev() {
            dp[j] += dp[j - num as usize];
        }
    }
    dp[target]
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(func(vec![1], 1), 1);
    }
    test(find_target_sum_ways);
    test(find_target_sum_ways2);
}
