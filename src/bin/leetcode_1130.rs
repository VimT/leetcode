//! 叶值的最小代价生成树

/// 翻译：将数组中相邻的数两两合并，计算他们的乘积之和，求最小的乘积之和。合并相邻的两个数之后得到的是较大的一个数。
/// dp[i][j] 表示将 arr[i...j] 合并之后所得的最小乘积之和。
/// dp[i][j]=min(dp[i][j], dp[i][k] + dp[k+1][j] + max[i][k]*max[k+1][j])
pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut max = vec![vec![0; len]; len];
    for i in 0..len {
        let mut cur_max = arr[i];
        for j in i..len {
            cur_max = cur_max.max(arr[j]);
            max[i][j] = cur_max;
        }
    }
    let mut dp = vec![vec![0; len]; len];
    for alen in 1..len {
        for i in 0..len - alen {
            let j = i + alen;
            dp[i][j] = i32::MAX;
            for k in i..j {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j] + max[i][k] * max[k + 1][j]);
            }
        }
    }
    dp[0][len - 1]
}

/// 单调栈，贪心
/// 先将两个更小的数合并得到的乘积之和 res 更小
pub fn mct_from_leaf_values_single_stack(mut arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut stack = vec![];
    let mut result = 0;
    for i in 0..len {
        while !stack.is_empty() && (*stack.last().unwrap() <= arr[i] || i == len - 1) {
            let bottom = stack.pop().unwrap();
            if !stack.is_empty() {
                result += arr[i].min(*stack.last().unwrap()) * bottom;
            } else {
                result += bottom * arr[i];
            }
            arr[i] = bottom.max(arr[i]);
        }
        stack.push(arr[i]);
    }
    result
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> i32) {
        assert_eq!(func(vec![6, 2, 4]), 32);
        assert_eq!(func(vec![4, 11]), 44);
    }
    test(mct_from_leaf_values);
    test(mct_from_leaf_values_single_stack);
}
