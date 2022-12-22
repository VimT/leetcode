//! 将节点分成尽可能多的组

use std::collections::VecDeque;

pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n + 1];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }

    let mut seen = vec![false; n + 1];

    let mut result = 0;
    let mut q = VecDeque::new();
    for i in 1..=n {
        if !seen[i] {

            // 找连通图
            let mut s = vec![];
            q.push_back(i);
            seen[i] = true;
            while !q.is_empty() {
                let u = q.pop_front().unwrap();
                s.push(u);
                for &v in &g[u] {
                    if !seen[v] {
                        q.push_back(v);
                        seen[v] = true;
                    }
                }
            }

            // 从任意源点出发，看能构成的最高的树。。这么暴力的吗。。
            let mut max = 0;
            for begin in s {
                q.push_back(begin);
                let mut dist = vec![0i32; n + 1];
                dist[begin] = 1;
                while !q.is_empty() {
                    let u = q.pop_front().unwrap();
                    max = max.max(dist[u]);
                    for &v in &g[u] {
                        if dist[v] == 0 {
                            dist[v] = dist[u] + 1;
                            q.push_back(v);
                        } else if (dist[u] - dist[v]).abs() != 1 {
                            return -1;
                        }
                    }
                }
            }
            result += max;
        }
    }

    result
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(26, vec![vec![9, 16], vec![8, 3], vec![20, 21], vec![12, 16], vec![14, 3], vec![7, 21], vec![22, 3], vec![22, 18], vec![11, 16], vec![25, 4], vec![2, 4], vec![14, 21], vec![23, 3], vec![17, 3], vec![2, 16], vec![24, 16], vec![13, 4], vec![10, 21], vec![7, 4], vec![9, 18], vec![14, 18], vec![14, 4], vec![14, 16], vec![1, 3], vec![25, 18], vec![17, 4], vec![1, 16], vec![23, 4], vec![2, 21], vec![5, 16], vec![24, 18], vec![20, 18], vec![19, 16], vec![24, 21], vec![9, 3], vec![24, 3], vec![19, 18], vec![25, 16], vec![19, 21], vec![6, 3], vec![26, 18], vec![5, 21], vec![20, 16], vec![2, 3], vec![10, 18], vec![26, 16], vec![8, 4], vec![11, 21], vec![23, 16], vec![13, 16], vec![25, 3], vec![7, 18], vec![19, 3], vec![20, 4], vec![26, 3], vec![23, 18], vec![15, 18], vec![17, 18], vec![10, 16], vec![26, 21], vec![23, 21], vec![7, 16], vec![8, 18], vec![10, 4], vec![24, 4], vec![7, 3], vec![11, 18], vec![9, 4], vec![26, 4], vec![13, 21], vec![22, 16], vec![22, 21], vec![20, 3], vec![6, 18], vec![9, 21], vec![10, 3], vec![22, 4], vec![1, 18], vec![25, 21], vec![11, 4], vec![1, 21], vec![15, 3], vec![1, 4], vec![15, 16], vec![2, 18], vec![13, 3], vec![8, 21], vec![13, 18], vec![11, 3], vec![15, 21], vec![8, 16], vec![17, 16], vec![15, 4], vec![12, 3], vec![6, 4], vec![17, 21], vec![5, 18], vec![6, 16], vec![6, 21], vec![12, 4], vec![19, 4], vec![5, 3], vec![12, 21], vec![5, 4]]), 4);
        assert_eq!(func(92, vec![vec![67, 29], vec![13, 29], vec![77, 29], vec![36, 29], vec![82, 29], vec![54, 29], vec![57, 29], vec![53, 29], vec![68, 29], vec![26, 29], vec![21, 29], vec![46, 29], vec![41, 29], vec![45, 29], vec![56, 29], vec![88, 29], vec![2, 29], vec![7, 29], vec![5, 29], vec![16, 29], vec![37, 29], vec![50, 29], vec![79, 29], vec![91, 29], vec![48, 29], vec![87, 29], vec![25, 29], vec![80, 29], vec![71, 29], vec![9, 29], vec![78, 29], vec![33, 29], vec![4, 29], vec![44, 29], vec![72, 29], vec![65, 29], vec![61, 29]]), 57);
        assert_eq!(func(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]), -1);
        assert_eq!(func(6, vec![vec![1, 2], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 3], vec![4, 6]]), 4);
    }
    test(magnificent_sets);
}
