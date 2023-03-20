//! 课程表 IV

use std::collections::VecDeque;

pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = num_courses as usize;
    let mut cache = vec![vec![None; n]; n];
    let mut g = vec![vec![]; n];
    for p in prerequisites {
        g[p[0] as usize].push(p[1] as usize);
        cache[p[0] as usize][p[1] as usize] = Some(true);
    }
    fn dfs(g: &Vec<Vec<usize>>, start: usize, end: usize, cache: &mut Vec<Vec<Option<bool>>>) -> bool {
        if let Some(ans) = cache[start][end] {
            return ans;
        }
        for &p in &g[start] {
            if dfs(g, p, end, cache) {
                cache[start][end] = Some(true);
                return true;
            }
        }
        cache[start][end] = Some(false);
        false
    }
    queries.into_iter().map(|q| {
        dfs(&g, q[0] as usize, q[1] as usize, &mut cache)
    }).collect()
}

/// bfs
pub fn check_if_prerequisite2(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = num_courses as usize;
    let mut g = vec![vec![]; n];
    for p in prerequisites {
        g[p[0] as usize].push(p[1] as usize);
    }
    let mut reach = vec![vec![false; n]; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        q.push_back(i);
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for &v in &g[u] {
                if !reach[i][v] {
                    reach[i][v] = true;
                    q.push_back(v);
                }
            }
        }
    }
    queries.into_iter().map(|q| {
        reach[q[0] as usize][q[1] as usize]
    }).collect()
}

/// floyd
pub fn check_if_prerequisite3(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = num_courses as usize;
    let mut dp = vec![vec![false; n]; n];
    for pre in prerequisites {
        dp[pre[0] as usize][pre[1] as usize] = true;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] |= dp[i][k] && dp[k][j];
            }
        }
    }
    queries.into_iter().map(|q| {
        dp[q[0] as usize][q[1] as usize]
    }).collect()
}

fn main() {
    fn test(func: fn(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool>) {
        assert_eq!(func(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]), vec![false, true]);
        assert_eq!(func(2, vec![], vec![vec![1, 0], vec![0, 1]]), vec![false, false]);
        assert_eq!(func(3, vec![vec![1, 2], vec![1, 0], vec![2, 0]], vec![vec![1, 0], vec![1, 2]]), vec![true, true]);
    }
    test(check_if_prerequisite);
    test(check_if_prerequisite2);
    test(check_if_prerequisite3);
}
