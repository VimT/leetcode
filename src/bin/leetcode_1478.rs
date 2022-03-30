//! 安排邮筒

// medsum: i->j 房屋到中位数房屋的和
// dp[i][j]  给前 i 栋房子（从 0 开始编号）安排 j 个邮筒（从 1 开始编号）的最小距离总和
pub fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    houses.sort_unstable();
    let len = houses.len();
    let mut medsum = vec![vec![0; len]; len];
    for i in (0..len - 1).rev() {
        for j in i + 1..len {
            medsum[i][j] = medsum[i + 1][j - 1] + houses[j] - houses[i];
        }
    }
    let mut dp = vec![vec![i32::MAX / 2; k + 1]; len];
    for i in 0..len {
        dp[i][1] = medsum[0][i];
        for j in 2..=k.min(i + 1) {
            for i0 in 0..i {
                dp[i][j] = dp[i][j].min(dp[i0][j - 1] + medsum[i0 + 1][i]);
            }
        }
    }
    dp[len - 1][k]
}

fn main() {
    assert_eq!(min_distance(vec![1, 4, 8, 10, 20], 3), 5);
    assert_eq!(min_distance(vec![2, 3, 5, 12, 18], 2), 9);
}
