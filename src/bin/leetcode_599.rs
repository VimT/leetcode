//! 两个列表的最小索引总和

use std::collections::HashMap;

use leetcode::svec;

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let set1: HashMap<String, usize> = list1.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
    let mut min_idx = usize::MAX;
    let mut result = vec![];
    for (idx2, w) in list2.into_iter().enumerate() {
        if let Some(&idx1) = set1.get(&w) {
            if idx1 + idx2 < min_idx {
                min_idx = idx1 + idx2;
                result.clear();
                result.push(w);
            } else if idx1 + idx2 == min_idx {
                result.push(w);
            }
        }
    }
    result
}

fn main() {
    assert_eq!(find_restaurant(svec!["Shogun", "Tapioca Express", "Burger King", "KFC"], svec!["Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"]), svec!["Shogun"]);
    assert_eq!(find_restaurant(svec!["Shogun", "Tapioca Express", "Burger King", "KFC"], svec!["KFC", "Shogun", "Burger King"]), svec!["Shogun"]);
}
