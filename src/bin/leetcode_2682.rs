//! 找出转圈游戏输家

pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
    let mut players: Vec<i32> = (1..=n).collect();
    let mut cur = 0;
    let mut i = 1;
    while players[cur] > 0 {
        players[cur] = 0;
        cur += i * k as usize;
        cur %= n as usize;
        i += 1;
    }
    players.into_iter().filter(|x| *x > 0).collect()
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> Vec<i32>) {
        assert_eq!(func(5, 2), vec![4, 5]);
        assert_eq!(func(4, 4), vec![2, 3, 4]);
    }
    test(circular_game_losers);
}
