//! 合并相似的物品

use std::collections::HashMap;

pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for item in items1 {
        *map.entry(item[0]).or_default() += item[1];
    }
    for item in items2 {
        *map.entry(item[0]).or_default() += item[1];
    }
    let mut result: Vec<Vec<i32>> = map.into_iter().map(|x| vec![x.0, x.1]).collect();
    result.sort_unstable();
    result
}

fn main() {
    assert_eq!(merge_similar_items(vec![vec![1, 1], vec![4, 5], vec![3, 8]], vec![vec![3, 1], vec![1, 5]]), vec![vec![1, 6], vec![3, 9], vec![4, 5]]);
    assert_eq!(merge_similar_items(vec![vec![1, 1], vec![3, 2], vec![2, 3]], vec![vec![2, 1], vec![3, 2], vec![1, 3]]), vec![vec![1, 4], vec![2, 4], vec![3, 4]]);
    assert_eq!(merge_similar_items(vec![vec![1, 3], vec![2, 2]], vec![vec![7, 1], vec![2, 2], vec![1, 4]]), vec![vec![1, 7], vec![2, 4], vec![7, 1]]);
}
