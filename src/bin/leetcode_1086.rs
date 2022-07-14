//! 前五科的均分

use std::collections::HashMap;

pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut score: HashMap<i32, Vec<i32>> = HashMap::new();
    for item in items {
        score.entry(item[0]).or_default().push(item[1]);
    }
    let mut result = vec![];
    for (student, mut scores) in score {
        scores.sort_unstable();
        result.push(vec![student, scores[scores.len() - 5..].iter().sum::<i32>() / 5]);
    }
    result.sort_unstable();
    result
}

fn main() {
    fn test(func: fn(items: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![1, 91], vec![1, 92], vec![2, 93], vec![2, 97], vec![1, 60], vec![2, 77], vec![1, 65], vec![1, 87], vec![1, 100], vec![2, 100], vec![2, 76]]), vec![vec![1, 87], vec![2, 88]]);
        assert_eq!(func(vec![vec![1, 100], vec![7, 100], vec![1, 100], vec![7, 100], vec![1, 100], vec![7, 100], vec![1, 100], vec![7, 100], vec![1, 100], vec![7, 100]]), vec![vec![1, 100], vec![7, 100]]);
    }
    test(high_five);
}
