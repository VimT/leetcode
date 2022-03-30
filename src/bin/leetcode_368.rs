//! 最大整除子集

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let len = nums.len();
    let mut dp: Vec<Vec<i32>> = nums.iter().map(|x| vec![*x]).collect();
    for i in 1..len {
        for j in 0..i {
            if nums[i] % nums[j] == 0 {
                if dp[j].len() + 1 > dp[i].len() {
                    dp[i] = dp[j].clone();
                    dp[i].push(nums[i]);
                }
            }
        }
    }
    let mut result = &vec![];
    let mut max = 0;
    for i in 0..len {
        if dp[i].len() > max {
            result = &dp[i];
            max = dp[i].len();
        }
    }
    result.clone()
}

fn main() {
    assert_eq!(largest_divisible_subset(vec![1, 2, 3]), vec![1, 2]);
    assert_eq!(largest_divisible_subset(vec![1, 2, 4, 8]), vec![1, 2, 4, 8]);
}
