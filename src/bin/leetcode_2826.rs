//! 将三个组排序

use std::cmp::Ordering;

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp = vec![[i32::MAX / 2; 3]; len + 1]; // dp[i][j] 表示将前 i 个数，改成以 j 结尾的最小步数
    dp[0].fill(0);
    for i in 0..len {
        for j in 0..3 {
            for before in 0..=j {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][before] + (nums[i] != (j as i32 + 1)) as i32);
            }
        }
    }
    *dp[len].iter().min().unwrap()
}


/// LIS 的做法 （递增变成非递减） O(nlogn)
pub fn minimum_operations2(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![];
    let len = nums.len();
    for num in nums {
        let j = dp.binary_search_by(|x| x.cmp(&num).then(Ordering::Less)).unwrap_err();
        if j == dp.len() {
            dp.push(num);
        } else {
            dp[j] = num;
        }
    }
    (len - dp.len()) as i32
}

/// dp的空间优化
pub fn minimum_operations3(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; 3];
    // dp[i][j] 表示将前 i 个数，改成以 j 结尾的最小步数
    for num in nums {
        for j in (0..3).rev() {
            dp[j] = (0..=j).map(|before| dp[before] + (num != (j as i32 + 1)) as i32).min().unwrap();
        }
    }
    dp.into_iter().min().unwrap()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 1, 3, 2, 1]), 3);
        assert_eq!(func(vec![1, 3, 2, 1, 3, 3]), 2);
        assert_eq!(func(vec![2, 2, 2, 2, 3, 3]), 0);
    }
    test(minimum_operations);
    test(minimum_operations2);
    test(minimum_operations3);
}
