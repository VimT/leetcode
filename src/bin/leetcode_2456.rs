//! 最流行的视频创作者

use std::collections::HashMap;
use leetcode::{svec, unorder};

pub fn most_popular_creator(creators: Vec<String>, ids: Vec<String>, views: Vec<i32>) -> Vec<Vec<String>> {
    let mut creator_view: HashMap<String, (i32, Vec<(i32, String)>)> = HashMap::new();
    for ((creator, id), view) in creators.into_iter().zip(ids).zip(views) {
        let entry = creator_view.entry(creator.clone()).or_default();
        entry.0 += view;
        entry.1.push((view, id));
    }
    let max = creator_view.values().map(|x| x.0).max().unwrap();
    creator_view.into_iter().filter(|x| x.1.0 == max).map(|(creator, (_, view_id))| {
        let mut max = -1;
        let mut max_id = String::new();
        for (view, id) in view_id {
            if view > max || (view == max && id < max_id) {
                max = view;
                max_id = id;
            }
        }
        vec![creator, max_id]
    }).collect()
}

fn main() {
    assert_eq!(unorder(most_popular_creator(svec!["alice", "bob", "alice", "chris"], svec!["one", "two", "three", "four"], vec![5, 10, 5, 4])), [svec!["alice", "one"], svec!["bob", "two"]]);
    assert_eq!(unorder(most_popular_creator(svec!["alice","alice","alice"], svec!["a","b","c"], vec![1, 2, 2])), [svec!["alice","b"]]);
}
