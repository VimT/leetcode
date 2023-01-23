//! 查找用户活跃分钟数

use std::collections::{HashMap, HashSet};

pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut cnt: HashMap<i32, HashSet<i32>> = HashMap::new();
    for log in logs {
        cnt.entry(log[0]).or_default().insert(log[1]);
    }
    let mut result = vec![0; k as usize];
    for (_, set) in cnt {
        result[set.len() - 1] += 1;
    }
    result
}

fn main() {
    fn test(func: fn(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32>) {
        assert_eq!(func(vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]], 5), vec![0, 2, 0, 0, 0]);
        assert_eq!(func(vec![vec![1, 1], vec![2, 2], vec![2, 3]], 4), vec![1, 1, 0, 0]);
    }
    test(finding_users_active_minutes);
}
