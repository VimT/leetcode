//! 石子游戏 III

use std::cmp::Ordering;

pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    fn dfs(s: &Vec<i32>, i: usize, cache: &mut Vec<i32>) -> i32 {
        if i >= s.len() { return 0; }
        if cache[i] != -1 {
            return cache[i];
        }
        let mut result = i32::MIN;
        let mut cur = 0;
        for j in i..(i + 3).min(s.len()) {
            cur += s[j];
            result = result.max(cur - dfs(s, j + 1, cache));
        }
        cache[i] = result;
        result
    }
    match dfs(&stone_value, 0, &mut vec![-1; stone_value.len()]).cmp(&0) {
        Ordering::Less => "Bob",
        Ordering::Equal => "Tie",
        Ordering::Greater => "Alice"
    }.to_string()
}

pub fn stone_game_iii2(stone_value: Vec<i32>) -> String {
    let len = stone_value.len();
    let mut dp = vec![i32::MIN; len + 1];
    dp[len] = 0;
    for i in (0..len).rev() {
        let mut cur = 0;
        for j in i..(i + 3).min(len) {
            cur += stone_value[j];
            dp[i] = dp[i].max(cur - dp[j + 1]);
        }
    }
    match dp[0].cmp(&0) {
        Ordering::Less => "Bob",
        Ordering::Equal => "Tie",
        Ordering::Greater => "Alice"
    }.to_string()
}

fn main() {
    fn test(func: fn(stone_value: Vec<i32>) -> String) {
        assert_eq!(func(vec![1, 2, 3, 7]), String::from("Bob"));
        assert_eq!(func(vec![1, 2, 3, -9]), String::from("Alice"));
        assert_eq!(func(vec![1, 2, 3, 6]), String::from("Tie"));
    }
    test(stone_game_iii);
    test(stone_game_iii2);
}
