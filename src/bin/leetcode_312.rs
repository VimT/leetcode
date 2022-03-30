//! 戳气球


/// inner(i, j) 表示 (i, j) 区间，所能获取到的最大值。
/// mid为(i,j)区间，第一个插入的气球， 递归计算 (i, mid) (j, mid) 的贡献，得到结果
pub fn max_coins(mut nums: Vec<i32>) -> i32 {
    fn inner(nums: &Vec<i32>, i: usize, j: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
        if mem[i][j] != -1 { return mem[i][j]; }
        if i >= j - 1 { return 0; }
        let mut ans = 0;
        for mid in i + 1..j {
            ans = ans.max(inner(nums, mid, j, mem) + inner(nums, i, mid, mem) + nums[i] * nums[mid] * nums[j]);
        }
        mem[i][j] = ans;
        ans
    }

    let len = nums.len();
    let mut mem = vec![vec![-1; len + 2]; len + 2];
    nums.insert(0, 1);
    nums.push(1);
    inner(&nums, 0, len + 1, &mut mem)
}


pub fn max_coins_dp(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    nums.insert(0, 1);
    nums.push(1);
    let mut dp = vec![vec![0; len + 2]; len + 2];
    for i in (0..len).rev() {
        for j in i + 2..=len + 1 {
            for k in i + 1..j {
                dp[i][j] = dp[i][j].max(dp[i][k] + dp[k][j] + nums[i] * nums[k] * nums[j]);
            }
        }
    }
    dp[0][len + 1]
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 1, 5, 8]), 167);
        assert_eq!(func(vec![1, 5]), 10);
    }
    test(max_coins);
    test(max_coins_dp);
}
