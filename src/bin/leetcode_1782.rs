//! 统计点对的数目

use std::collections::HashMap;

/// 双指针
pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let n = n as usize;
    let mut deg = vec![0; n];
    let mut cnt: HashMap<(i32, i32), i32> = HashMap::new();
    for edge in edges {
        let (x, y) = if edge[0] > edge[1] { (edge[1] - 1, edge[0] - 1) } else { (edge[0] - 1, edge[1] - 1) };
        deg[x as usize] += 1;
        deg[y as usize] += 1;
        *cnt.entry((x, y)).or_default() += 1;
    }
    let mut arr = deg.clone();
    arr.sort_unstable();
    queries.into_iter().map(|q| {
        let mut total = 0;
        let mut j = n - 1;
        for i in 0..n {
            while j > i && arr[i] + arr[j] > q {
                j -= 1;
            }
            total += n - 1 - i.max(j);
        }
        for (&(x, y), &freq) in &cnt {
            if deg[x as usize] + deg[y as usize] > q && deg[x as usize] + deg[y as usize] - freq <= q {
                total -= 1;
            }
        }
        total as i32
    }).collect()
}

/// 双指针优化
pub fn count_pairs2(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let n = n as usize;
    let mut deg = vec![0; n + 1];
    let mut edge_cnt: HashMap<(i32, i32), i32> = HashMap::new();
    for edge in edges {
        let (x, y) = if edge[0] > edge[1] { (edge[1], edge[0]) } else { (edge[0], edge[1]) };
        deg[x as usize] += 1;
        deg[y as usize] += 1;
        *edge_cnt.entry((x, y)).or_default() += 1;
    }
    let mut deg_cnt: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;
    for &num in &deg[1..] {
        *deg_cnt.entry(num).or_default() += 1;
        max = max.max(num);
    }

    let mut cnts = vec![0; max as usize * 2 + 2];
    for (&deg1, &c1) in &deg_cnt {
        for (&deg2, &c2) in &deg_cnt {
            if deg1 < deg2 {
                cnts[(deg1 + deg2) as usize] += c1 * c2;
            } else if deg1 == deg2 {
                cnts[(deg1 + deg2) as usize] += c1 * (c1 - 1) / 2;
            }
        }
    }

    for ((x, y), c) in edge_cnt {
        let s = deg[x as usize] + deg[y as usize];
        cnts[s as usize] -= 1;
        cnts[(s - c) as usize] += 1;
    }

    // 计算cnts 的后缀和
    for i in (1..cnts.len()).rev() {
        cnts[i - 1] += cnts[i];
    }
    queries.into_iter().map(|q| cnts[(q as usize + 1).min(cnts.len() - 1)]).collect()
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(4, vec![vec![1, 2], vec![2, 4], vec![1, 3], vec![2, 3], vec![2, 1]], vec![2, 3]), vec![6, 5]);
        assert_eq!(func(5, vec![vec![1, 5], vec![1, 5], vec![3, 4], vec![2, 5], vec![1, 3], vec![5, 1], vec![2, 3], vec![2, 5]], vec![1, 2, 3, 4, 5]), vec![10, 10, 9, 8, 6]);
    }
    test(count_pairs);
    test(count_pairs2);
}
