//! 求出胜利玩家的数目

pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
    let mut players = vec![vec![0; 11]; n as usize];
    for p in pick {
        players[p[0] as usize][p[1] as usize] += 1;
    }
    let mut result = 0;
    for (player, i) in players.into_iter().zip(0..) {
        if player.into_iter().any(|x| x > i) {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, pick: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(4, vec![vec![0, 0], vec![1, 0], vec![1, 0], vec![2, 1], vec![2, 1], vec![2, 0]]), 2);
        assert_eq!(func(5, vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4]]), 0);
        assert_eq!(func(5, vec![vec![1, 1], vec![2, 4], vec![2, 4], vec![2, 4]]), 1);
    }
    test(winning_player_count);
}
