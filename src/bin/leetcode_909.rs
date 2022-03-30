//! 蛇梯棋

use std::collections::BinaryHeap;

pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let n = board.len();
    let to_num = |x: usize, y: usize| -> usize {
        let dx = n - x - 1;
        return if dx & 1 == 0 {
            // left to right
            dx * n + 1 + y
        } else {
            // right to left
            n * (dx + 1) - y
        };
    };
    let mut quick = vec![0; n * n + 1];
    for i in 0..n {
        for j in 0..n {
            if board[i][j] != -1 {
                quick[to_num(i, j)] = board[i][j] as usize;
            }
        }
    }
    let target = n * n;
    let mut heap = BinaryHeap::new();
    heap.push((0, 1));
    let mut min_dis = vec![i32::MAX; n * n + 1];
    min_dis[1] = 0;
    while !heap.is_empty() {
        let (dis, mut node) = heap.pop().unwrap();
        if node == target {
            return -dis;
        }
        if min_dis[node] < -dis { continue; }
        let end = (node + 6).min(target);
        node += 1;
        while node <= end {
            let t = if quick[node] != 0 { quick[node] } else { node };
            if min_dis[t] > -dis + 1 {
                min_dis[t] = -dis + 1;
                heap.push((dis - 1, t));
            }
            node += 1;
        }
    }
    -1
}

fn main() {
    assert_eq!(snakes_and_ladders(vec![vec![1, 1, -1], vec![1, 1, 1], vec![-1, 1, 1]]), -1);
    assert_eq!(snakes_and_ladders(vec![vec![-1, -1, -1, -1, -1, -1], vec![-1, -1, -1, -1, -1, -1], vec![-1, -1, -1, -1, -1, -1], vec![-1, 35, -1, -1, 13, -1], vec![-1, -1, -1, -1, -1, -1], vec![-1, 15, -1, -1, -1, -1]]), 4);
    assert_eq!(snakes_and_ladders(vec![vec![-1, -1], vec![-1, 3]]), 1);
}
