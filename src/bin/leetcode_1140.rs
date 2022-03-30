//! 石子游戏 II

/// dp[i][j] 从i开始取，j是这一轮的x
pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let len = piles.len();
    let mut sum = 0;
    let mut dp = vec![vec![0; len + 1]; len];
    for i in (0..len).rev() {
        sum += piles[i];
        for j in 1..=len {
            dp[i][j] = if i + j * 2 >= len {
                sum
            } else {
                let mut next_min = i32::MAX;
                for x in 1..=2 * j {
                    next_min = next_min.min(dp[i + x][x.max(j)])
                }
                sum - next_min
            }
        }
    }
    dp[0][1]
}

fn main() {
    fn test(func: fn(piles: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 7, 9, 4, 4]), 10);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 100]), 104);
    }
    test(stone_game_ii);
}
