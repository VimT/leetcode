//! 切棍子的最小成本

/// dp[i][j] 表示切割点从i到j的最小切割成本
pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
    cuts.push(0);
    cuts.push(n);
    cuts.sort_unstable();

    let len = cuts.len();
    let mut dp = vec![vec![i32::MAX / 2; len]; len];
    for i in 0..len - 1 {
        dp[i][i + 1] = 0;
    }

    for cut_len in 2..len {
        for i in 0..len - cut_len {
            let j = i + cut_len;
            for cut in i + 1..j {
                dp[i][j] = dp[i][j].min(dp[i][cut] + dp[cut][j] + cuts[j] - cuts[i]);
            }
        }
    }

    dp[0][len - 1]
}

fn main() {
    fn test(func: fn(n: i32, cuts: Vec<i32>) -> i32) {
        assert_eq!(func(7, vec![1, 3, 4, 5]), 16);
        assert_eq!(func(9, vec![5, 6, 1, 4, 2]), 22);
    }
    test(min_cost);
}
