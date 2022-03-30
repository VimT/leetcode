//! 喧闹和富有

use std::collections::HashMap;

pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    let len = quiet.len();

    let mut m: HashMap<usize, Vec<usize>> = HashMap::new();
    for rich in richer {
        m.entry(rich[1] as usize).or_default().push(rich[0] as usize);
    }
    fn dfs(m: &mut HashMap<usize, Vec<usize>>, quiet: &Vec<i32>, cur: usize, cache: &mut Vec<Option<(usize, i32)>>) -> (usize, i32) {
        if let Some(x) = cache[cur] {
            return x;
        }
        let mut min = (cur, quiet[cur]);
        if m.get(&cur).is_some() {
            let nexts = m.get(&cur).unwrap().clone();
            for next in nexts {
                let next_val = dfs(m, quiet, next, cache);
                if next_val.1 < min.1 {
                    min = next_val;
                }
            }
        }
        cache[cur] = Some(min);
        min
    }

    let mut result = vec![0; len];
    let mut cache = vec![None; len];
    for i in 0..len {
        result[i] = dfs(&mut m, &quiet, i, &mut cache).0 as i32;
    }
    result
}

pub fn loud_and_rich_topo_sort(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    let len = quiet.len();
    let mut m = vec![vec![false; len]; len];
    let mut indegree = vec![0; len];
    for rich in richer {
        m[rich[0] as usize][rich[1] as usize] = true;
        indegree[rich[1] as usize] += 1;
    }
    let mut q = vec![];
    for i in 0..len {
        if indegree[i] == 0 {
            q.push(i);
        }
    }
    let mut result: Vec<i32> = (0..len as i32).collect();
    while !q.is_empty() {
        let u = q.pop().unwrap();
        for v in 0..len {
            if m[u][v] {
                if quiet[result[v] as usize] > quiet[result[u] as usize] {
                    result[v] = result[u];
                }
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    q.push(v);
                }
            }
        }
    }
    result
}

fn main() {
    assert_eq!(loud_and_rich_topo_sort(vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![3, 7], vec![4, 3], vec![5, 3], vec![6, 3]], vec![3, 2, 5, 4, 6, 1, 7, 0]), vec![5, 5, 2, 5, 4, 5, 6, 7]);
    assert_eq!(loud_and_rich_topo_sort(vec![], vec![0]), vec![0]);
}
