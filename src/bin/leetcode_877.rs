//! 石子游戏

/// same as 486
/// dp[i][j]：作为先手，在区间 nums[i..j] 里进行选择可以获得的相对分数
/// 数学做法: 先手只要永远选择奇数/偶数堆里较大的一个就能赢。
pub fn stone_game(piles: Vec<i32>) -> bool {
    let len = piles.len();
    let mut dp = vec![vec![i32::MIN; len]; len];
    for i in 0..len {
        dp[i][i] = piles[i];
    }
    for i in (0..len - 1).rev() {
        for j in i + 1..len {
            dp[i][j] = (piles[i] - dp[i + 1][j]).max(piles[j] - dp[i][j - 1]);
        }
    }
    dp[0][len - 1] > 0
}

fn main() {
    assert_eq!(stone_game(vec![5, 3, 4, 5]), true);
    assert_eq!(stone_game(vec![3, 7, 2, 3]), true);
}
