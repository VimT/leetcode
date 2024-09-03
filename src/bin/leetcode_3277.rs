//! 查询子数组最大异或值

/// 两次区间dp
pub fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = nums.len();
    let mut dp = vec![vec![0; len]; len];
    let mut max_xor = vec![vec![0; len]; len];
    for win in 1..=len {
        for i in 0..=len - win {
            let j = i + win - 1;
            dp[i][j] = if i == j { nums[i] } else { dp[i][j - 1] ^ dp[i + 1][j] };
            max_xor[i][j] = if i == j { dp[i][j] } else { max_xor[i][j].max(max_xor[i][j - 1]).max(max_xor[i + 1][j]).max(dp[i][j]) };
        }
    }
    queries.into_iter().map(|query| max_xor[query[0] as usize][query[1] as usize]).collect()
}

/// 区间 dp 的另一种写法
pub fn maximum_subarray_xor2(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = nums.len();
    let mut dp = vec![vec![0; len]; len];
    let mut mx = vec![vec![0; len]; len];
    for i in (0..len).rev() {
        mx[i][i] = nums[i];
        dp[i][i] = nums[i];
        for j in i + 1..len {
            dp[i][j] = dp[i][j - 1] ^ dp[i + 1][j];
            mx[i][j] = mx[i][j].max(mx[i][j - 1]).max(mx[i + 1][j]).max(dp[i][j]);
        }
    }
    queries.into_iter().map(|query| mx[query[0] as usize][query[1] as usize]).collect()
}


fn main() {
    fn test(func: fn(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![2, 8, 4, 32, 16, 1], vec![vec![0, 2], vec![1, 4], vec![0, 5]]), vec![12, 60, 60]);
        assert_eq!(func(vec![0, 7, 3, 2, 8, 5, 1], vec![vec![0, 3], vec![1, 5], vec![2, 4], vec![2, 6], vec![5, 6]]), vec![7, 14, 11, 14, 5]);
    }
    test(maximum_subarray_xor);
    test(maximum_subarray_xor2);
}
