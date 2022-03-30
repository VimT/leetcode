//! 合法重新排列数对

use std::collections::HashMap;

/// 欧拉图  Hierholzer 算法
pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut degree: HashMap<i32, i32> = HashMap::new();
    for pair in &pairs {
        m.entry(pair[0]).or_default().push(pair[1]);
        *degree.entry(pair[1]).or_default() += 1;
        *degree.entry(pair[0]).or_default() -= 1;
    }
    let mut cur = pairs[0][0];
    for (k, v) in degree {
        if v == -1 {
            cur = k;
            break;
        }
    }
    let mut result = vec![];
    let mut s = vec![];
    s.push(cur);
    while !s.is_empty() {
        if let Some(v) = m.get_mut(&cur) {
            if v.is_empty() {
                result.push(cur);
                cur = s.pop().unwrap();
            } else {
                s.push(cur);
                cur = v.pop().unwrap();
            }
        } else {
            result.push(cur);
            cur = s.pop().unwrap();
        }
    }
    result.reverse();
    result.windows(2).map(|x| vec![x[0], x[1]]).collect()
}

pub fn valid_arrangement_dfs(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    unsafe fn dfs(m: &mut HashMap<i32, Vec<i32>>, cur: i32, ans: &mut Vec<Vec<i32>>) {
        if m.get(&cur).is_none() { return; }
        let v = m.get_mut(&cur).unwrap() as *mut Vec<i32>;
        while !(*v).is_empty() {
            let nx = (*v).pop().unwrap();
            dfs(m, nx, ans);
            ans.push(vec![cur, nx]);
        }
    }
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut degree: HashMap<i32, i32> = HashMap::new();
    for pair in &pairs {
        m.entry(pair[0]).or_default().push(pair[1]);
        *degree.entry(pair[1]).or_default() += 1;
        *degree.entry(pair[0]).or_default() -= 1;
    }
    let mut cur = pairs[0][0];
    for (k, v) in degree {
        if v == -1 {
            cur = k;
            break;
        }
    }
    let mut result = Vec::with_capacity(pairs.len());
    unsafe { dfs(&mut m, cur, &mut result) }
    result.reverse();
    result
}

fn main() {
    assert_eq!(valid_arrangement_dfs(vec![vec![1, 2], vec![1, 3], vec![2, 1]]), vec![vec![1, 2], vec![2, 1], vec![1, 3]]);
    assert_eq!(valid_arrangement_dfs(vec![vec![8, 5], vec![8, 7], vec![0, 8], vec![0, 5], vec![7, 0], vec![5, 0], vec![0, 7], vec![8, 0], vec![7, 8]]), vec![vec![8, 0], vec![0, 7], vec![7, 8], vec![8, 7], vec![7, 0], vec![0, 5], vec![5, 0], vec![0, 8], vec![8, 5]]);
    assert_eq!(valid_arrangement_dfs(vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]]), vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1]]);
    assert_eq!(valid_arrangement_dfs(vec![vec![1, 3], vec![3, 2], vec![2, 1]]), vec![vec![1, 3], vec![3, 2], vec![2, 1]]);
    assert_eq!(valid_arrangement_dfs(vec![vec![1, 1], vec![1, 1], vec![1, 1]]), vec![vec![1, 1], vec![1, 1], vec![1, 1]]);
}
