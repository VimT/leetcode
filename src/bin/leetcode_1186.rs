//! 删除一次得到子数组最大和

/// d[i][j] 表示以i为后缀，j=0表示已经用过，j=1表示还没用。考虑全负的特殊情况
pub fn maximum_sum(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut result = i32::MIN;
    let mut dp = vec![[0, 0]; len + 1];
    let mut max = i32::MIN;
    for i in 0..len {
        max = max.max(arr[i]);
        dp[i + 1][0] = (dp[i][0] + arr[i]).max(dp[i][1]);
        dp[i + 1][1] = (dp[i][1] + arr[i]).max(0);
        result = result.max(dp[i + 1][0]).max(dp[i + 1][1]);
    }
    if max < 0 { max } else { result }
}

/// 空间复杂度优化
pub fn maximum_sum_optimise(arr: Vec<i32>) -> i32 {
    let mut not_delete = 0;
    let mut after_delete = i32::MIN / 2;
    let mut result = i32::MIN;
    let mut max = i32::MIN;
    for num in arr {
        max = max.max(num);
        after_delete = (after_delete + num).max(not_delete);
        not_delete = (not_delete + num).max(0);
        result = result.max(not_delete).max(after_delete);
    }
    if max < 0 { max } else { result }
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, -2, 0, 3]), 4);
        assert_eq!(func(vec![1, -2, -2, 3]), 3);
        assert_eq!(func(vec![-1, -1, -1, -1]), -1);
    }
    test(maximum_sum);
    test(maximum_sum_optimise);
}
