//! 卡牌分组

use std::collections::HashMap;

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { return b; }
    gcd(b % a, a)
}

pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    let mut m = HashMap::new();
    for i in deck {
        *m.entry(i).or_insert(0i32) += 1;
    }
    let cnts: Vec<&i32> = m.values().collect();

    let mut min = -1;
    for &v in cnts {
        if min == -1 {
            min = v;
        } else {
            min = gcd(min, v);
        }
    }
    min >= 2
}

fn main() {
    assert_eq!(has_groups_size_x(vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 2]), true);
    assert_eq!(has_groups_size_x(vec![1]), false);
    assert_eq!(has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]), true);
    assert_eq!(has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]), false);
}
