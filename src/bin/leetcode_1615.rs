//! 最大网络秩

pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut edge = vec![vec![false; n]; n];
    let mut cnt = vec![0; n];
    for road in roads {
        cnt[road[0] as usize] += 1;
        cnt[road[1] as usize] += 1;
        edge[road[0] as usize][road[1] as usize] = true;
        edge[road[1] as usize][road[0] as usize] = true;
    }
    let mut result = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let mut tmp = cnt[i] + cnt[j];
            if edge[i][j] { tmp -= 1; }
            result = result.max(tmp);
        }
    }
    result
}

fn main() {
    assert_eq!(maximal_network_rank(4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]]), 4);
    assert_eq!(maximal_network_rank(5, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3], vec![2, 3], vec![2, 4]]), 5);
    assert_eq!(maximal_network_rank(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4], vec![5, 6], vec![5, 7]]), 5);
}

