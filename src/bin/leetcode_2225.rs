//! 找出输掉零场或一场比赛的玩家

use std::collections::HashMap;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut loss = HashMap::new();
    for m in matches {
        loss.entry(m[0]).or_default();
        *loss.entry(m[1]).or_insert(0) += 1;
    }
    let mut loss0 = vec![];
    let mut loss1 = vec![];
    for (k, v) in loss {
        if v == 0 {
            loss0.push(k);
        } else if v == 1 {
            loss1.push(k);
        }
    }
    loss0.sort_unstable();
    loss1.sort_unstable();
    vec![loss0, loss1]
}

fn main() {
    assert_eq!(find_winners(vec![vec![1, 3], vec![2, 3], vec![3, 6], vec![5, 6], vec![5, 7], vec![4, 5], vec![4, 8], vec![4, 9], vec![10, 4], vec![10, 9]]), vec![vec![1, 2, 10], vec![4, 5, 7, 8]]);
    assert_eq!(find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]]), vec![vec![1, 2, 5, 6], vec![]]);
}
