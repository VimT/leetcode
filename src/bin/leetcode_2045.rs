//! 到达目的地的第二短时间

use std::collections::VecDeque;

pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
    let n = n as usize;
    let mut e = vec![vec![]; n + 1];
    for edge in edges {
        e[edge[0] as usize].push(edge[1] as usize);
        e[edge[1] as usize].push(edge[0] as usize);
    }
    let mut first = vec![-1; n + 1];
    let mut second = vec![-1; n + 1];
    let mut q = VecDeque::new();
    q.push_back((1, 0));
    first[1] = 00;
    while !q.is_empty() {
        let (node, dao_time) = q.pop_front().unwrap();
        let mut chufa_time = dao_time;
        if (chufa_time / change) % 2 == 1 {
            chufa_time = (chufa_time / change + 1) * change;
        }
        let next_time = chufa_time + time;
        for &next in &e[node] {
            if first[next] == -1 {
                first[next] = next_time;
                q.push_back((next, next_time));
            } else if second[next] == -1 && next_time > first[next] {
                second[next] = next_time;
                q.push_back((next, next_time));
            }
        }
    }
    // println!("{:?}", first);
    // println!("{:?}", second);
    second[n]
}

fn main() {
    assert_eq!(second_minimum(5, vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]], 3, 5), 13);
    assert_eq!(second_minimum(2, vec![vec![1, 2]], 3, 2), 11);
}
