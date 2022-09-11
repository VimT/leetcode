//! 用户网站访问行为分析

use std::collections::{HashMap, HashSet};
use leetcode::svec;

pub fn most_visited_pattern(username: Vec<String>, timestamp: Vec<i32>, website: Vec<String>) -> Vec<String> {
    let mut website_map = HashMap::new();
    let mut websites = vec![];
    for ws in &website {
        if !website_map.contains_key(ws) {
            website_map.insert(ws.clone(), 0);
            websites.push(ws.clone());
        }
    }
    websites.sort_unstable();
    for (i, ws) in websites.iter().enumerate() {
        *website_map.get_mut(ws).unwrap() = i;
    }
    let mut table = HashMap::new();
    for ((un, ts), ws) in username.into_iter().zip(timestamp.into_iter()).zip(website.into_iter()) {
        table.entry(un).or_insert(vec![]).push((ts, website_map[&ws]));
    }
    let mut access_mode = HashMap::new();
    for (_, mut access) in table {
        if access.len() < 3 { continue; }
        access.sort_unstable();
        let len = access.len();
        let mut mode_set = HashSet::new();
        for i in 0..len {
            for j in i + 1..len {
                for k in j + 1..len {
                    mode_set.insert((access[i].1, access[j].1, access[k].1));
                }
            }
        }
        for mode in mode_set {
            *access_mode.entry(mode).or_insert(0) += 1;
        }
    }
    let mut result = (0, 0, 0);
    let mut max = 0;
    for (k, v) in access_mode {
        if v > max || (v == max && k < result) {
            result = k;
            max = v;
        }
    }
    vec![websites[result.0].clone(), websites[result.1].clone(), websites[result.2].clone()]
}

fn main() {
    fn test(func: fn(username: Vec<String>, timestamp: Vec<i32>, website: Vec<String>) -> Vec<String>) {
        assert_eq!(func(svec!["h","eiy","cq","h","cq","txldsscx","cq","txldsscx","h","cq","cq"],
                        vec![527896567, 334462937, 517687281, 134127993, 859112386, 159548699, 51100299, 444082139, 926837079, 317455832, 411747930],
                        svec!["hibympufi","hibympufi","hibympufi","hibympufi","hibympufi","hibympufi","hibympufi","hibympufi","yljmntrclw","hibympufi","yljmntrclw"]), vec!["hibympufi", "hibympufi", "yljmntrclw"]);
        assert_eq!(func(svec!["joe","joe","joe","james","james","james","james","mary","mary","mary"], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], svec!["home","about","career","home","cart","maps","home","home","about","career"]), vec!["home", "about", "career"]);
        assert_eq!(func(svec!["ua","ua","ua","ub","ub","ub"], vec![1, 2, 3, 4, 5, 6], svec!["a","b","a","a","b","c"]), vec!["a", "b", "a"]);
    }
    test(most_visited_pattern);
}
