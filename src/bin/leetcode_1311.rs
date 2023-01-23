//! 获取你好友已观看的视频

use std::collections::HashMap;
use leetcode::svec;

pub fn watched_videos_by_friends(watched_videos: Vec<Vec<String>>, g: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<String> {
    let len = g.len();
    let mut s = vec![false; len];
    s[id as usize] = true;
    let mut seen = vec![false; len];
    seen[id as usize] = true;
    for _ in 0..level {
        let mut ns = vec![false; len];
        for id in 0..len {
            if s[id] {
                for &nid in &g[id] {
                    if !seen[nid as usize] {
                        ns[nid as usize] = true;
                        seen[nid as usize] = true;
                    }
                }
            }
        }
        s = ns;
    }
    let mut m: HashMap<&String, i32> = HashMap::new();
    for i in 0..len {
        if s[i] {
            for video in &watched_videos[i] {
                *m.entry(video).or_default() += 1;
            }
        }
    }
    let mut v: Vec<(i32, &String)> = m.into_iter().map(|x| (x.1, x.0)).collect();
    v.sort_unstable();
    v.into_iter().map(|x| x.1.clone()).collect()
}

fn main() {
    fn test(func: fn(watched_videos: Vec<Vec<String>>, friends: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<String>) {
        assert_eq!(func(vec![svec!["A","B"], svec!["C"], svec!["B","C"], svec!["D"]], vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]], 0, 2), vec!["D"]);
        assert_eq!(func(vec![svec!["A","B"], svec!["C"], svec!["B","C"], svec!["D"]], vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]], 0, 1), vec!["B", "C"]);
    }
    test(watched_videos_by_friends);
}
