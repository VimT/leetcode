//! 最优账单平衡

use std::collections::HashMap;

pub fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
    let mut person: HashMap<i32, i32> = HashMap::new();
    for tra in transactions {
        *person.entry(tra[0]).or_default() -= tra[2];
        *person.entry(tra[1]).or_default() += tra[2];
    }
    let mut accounts: Vec<i32> = person.into_iter().map(|(_, v)| v).filter(|x| *x != 0).collect();
    fn dfs(accounts: &mut Vec<i32>, i: usize, cnt: i32, result: &mut i32) {
        if cnt > *result { return; }
        if i == accounts.len() {
            *result = (*result).min(cnt);
            return;
        }
        if accounts[i] == 0 {
            dfs(accounts, i + 1, cnt, result)
        } else {
            for j in i + 1..accounts.len() {
                if accounts[i] * accounts[j] < 0 {
                    accounts[j] += accounts[i];
                    dfs(accounts, i + 1, cnt + 1, result);
                    accounts[j] -= accounts[i];
                }
            }
        }
    }
    let mut result = i32::MAX;
    dfs(&mut accounts, 0, 0, &mut result);
    result
}

fn main() {
    fn test(func: fn(transactions: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![2, 0, 5], vec![3, 4, 4]]), 2);
        assert_eq!(func(vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 0, 1]]), 0);
        assert_eq!(func(vec![vec![0, 1, 10], vec![2, 0, 5]]), 2);
        assert_eq!(func(vec![vec![0, 1, 10], vec![1, 0, 1], vec![1, 2, 5], vec![2, 0, 5]]), 1);
    }
    test(min_transfers);
}
