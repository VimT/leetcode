//! 石子游戏 IV

pub fn winner_square_game(n: i32) -> bool {
    let mut dp = vec![false; n as usize + 1];
    dp[1] = true;
    for i in 2..=n as usize {
        let mut j = 1;
        while j * j <= i {
            if !dp[i - j * j] {
                dp[i] = true;
                break;
            }
            j += 1;
        }
    }
    dp[n as usize]
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(1), true);
        assert_eq!(func(2), false);
        assert_eq!(func(4), true);
    }
    test(winner_square_game);
}
