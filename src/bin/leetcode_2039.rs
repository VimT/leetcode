//! 网络空闲的时刻

pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
    let len = patience.len();
    let mut edge_map = vec![vec![]; len];
    for edge in edges {
        edge_map[edge[0] as usize].push(edge[1] as usize);
        edge_map[edge[1] as usize].push(edge[0] as usize);
    }
    let mut queue = vec![0];
    let mut dis = vec![i32::MAX; len];
    let mut cur_dis = 0;
    while !queue.is_empty() {
        let mut new_q = vec![];
        for node in queue {
            if dis[node] != i32::MAX { continue; }
            dis[node] = cur_dis;
            for &next in &edge_map[node] {
                new_q.push(next);
            }
        }
        queue = new_q;
        cur_dis += 1;
    }

    let mut result = 0;
    // println!("{:?}", dis);
    // println!("{:?}", patience);
    for i in 0..len {
        let node_max = if 2 * dis[i] <= patience[i] {
            2 * dis[i]
        } else {
            if 2 * dis[i] % patience[i] == 0 {
                2 * dis[i] + 2 * dis[i] / patience[i] * patience[i] - patience[i]
            } else {
                2 * dis[i] + 2 * dis[i] / patience[i] * patience[i]
            }
        };
        // println!("{}: {}", i, node_max);
        result = result.max(node_max);
    }
    result + 1
}

fn main() {
    assert_eq!(network_becomes_idle(vec![vec![0, 1], vec![1, 2]], vec![0, 2, 1]), 8);
    assert_eq!(network_becomes_idle(vec![vec![3, 8], vec![4, 13], vec![0, 7], vec![0, 4], vec![1, 8], vec![14, 1], vec![7, 2], vec![13, 10], vec![9, 11], vec![12, 14], vec![0, 6], vec![2, 12], vec![11, 5], vec![6, 9], vec![10, 3]],
                                    vec![0, 3, 2, 1, 5, 1, 5, 5, 3, 1, 2, 2, 2, 2, 1]), 20);
    assert_eq!(network_becomes_idle(vec![vec![0, 1], vec![0, 2], vec![1, 2]], vec![0, 10, 10]), 3);
}
