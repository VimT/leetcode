//! 按身高排序

use leetcode::svec;

pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut vn: Vec<(String, i32)> = names.into_iter().zip(heights).collect();
    vn.sort_unstable_by_key(|x| -x.1);
    vn.into_iter().map(|x| x.0).collect()
}

fn main() {
    assert_eq!(sort_people(svec!["Mary","John","Emma"], vec![180, 165, 170]), vec!["Mary", "Emma", "John"]);
    assert_eq!(sort_people(svec!["Alice","Bob","Bob"], vec![155, 185, 150]), vec!["Bob", "Alice", "Bob"]);
}
