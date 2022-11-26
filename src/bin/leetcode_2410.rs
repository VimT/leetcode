//! 运动员和训练师的最大匹配数


use std::collections::BTreeMap;

/// 贪心
pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
    let mut t: BTreeMap<i32, i32> = BTreeMap::new();
    for trainer in trainers {
        *t.entry(trainer).or_default() += 1;
    }
    players.into_iter().filter(|&x| {
        if let Some((&a, &b)) = t.range(x..).next() {
            if b == 1 {
                t.remove(&a);
            } else {
                t.insert(a, b - 1);
            }
            true
        } else { false }
    }).count() as i32
}

/// 贪心，双排序，不需要BTreeMap
pub fn match_players_and_trainers2(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
    players.sort_unstable();
    trainers.sort_unstable();
    let mut result = 0;
    let mut i = 0;
    let mut j = 0;
    while i < players.len() && j < trainers.len() {
        while j < trainers.len() && players[i] > trainers[j] { j += 1; }
        if j < trainers.len() && players[i] <= trainers[j] {
            result += 1;
            j += 1;
        }
        i += 1;
    }
    result
}

fn main() {
    fn test(func: fn(players: Vec<i32>, trainers: Vec<i32>) -> i32) {
        assert_eq!(func(vec![4, 7, 9], vec![8, 2, 5, 8]), 2);
        assert_eq!(func(vec![1, 1, 1], vec![10]), 1);
    }
    test(match_players_and_trainers);
    test(match_players_and_trainers2);
}
