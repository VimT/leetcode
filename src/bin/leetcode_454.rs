//! 四数相加 II

use std::collections::HashMap;

pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;

    for i in a {
        for &j in &b {
            map.entry(i + j).and_modify(|x| *x += 1).or_insert(1);
        }
    }
    for i in c {
        for &j in &d {
            if map.contains_key(&-(i + j)) {
                ans += map[&-(i + j)];
            }
        }
    }
    ans
}


fn main() {
    assert_eq!(four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]), 2);
    assert_eq!(four_sum_count(vec![0], vec![0], vec![0], vec![0]), 1);
}
