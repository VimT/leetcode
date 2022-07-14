//! 价格范围内最高排名的 K 样物品

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn highest_ranked_k_items(grid: Vec<Vec<i32>>, pricing: Vec<i32>, start: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut q = vec![];
    let k = k as usize;
    let mut heap = BinaryHeap::with_capacity(k * 2);
    let mut vis = vec![vec![false; n]; m];
    q.push((start[0] as usize, start[1] as usize));
    vis[start[0] as usize][start[1] as usize] = true;
    let mut step = 1;
    let start_price = grid[start[0] as usize][start[1] as usize];
    if start_price >= pricing[0] && start_price <= pricing[1] {
        heap.push(Reverse((0, start_price, start[0], start[1])));
    }
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while !q.is_empty() {
        let mut nq = Vec::with_capacity(q.len() * 2);
        for (x, y) in q {
            for (dx, dy) in DIR {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if grid[nx][ny] > 0 && !vis[nx][ny] {
                        vis[nx][ny] = true;
                        if grid[nx][ny] >= pricing[0] && grid[nx][ny] <= pricing[1] {
                            heap.push(Reverse((step, grid[nx][ny], nx as i32, ny as i32)));
                        }
                        nq.push((nx, ny));
                    }
                }
            }
        }
        if heap.len() >= k {
            break;
        }
        q = nq;
        step += 1;
    }
    let mut result = Vec::with_capacity(k);
    while !heap.is_empty() && result.len() < k {
        let Reverse(pop) = heap.pop().unwrap();
        result.push(vec![pop.2, pop.3]);
    }
    result
}

fn main() {
    assert_eq!(highest_ranked_k_items(vec![vec![1, 0, 1], vec![3, 5, 2], vec![1, 0, 1]],
                                      vec![2, 5],
                                      vec![1, 1],
                                      9), vec![vec![1, 1], vec![1, 2], vec![1, 0]]);
    assert_eq!(highest_ranked_k_items(vec![vec![1, 2, 0, 1], vec![1, 3, 0, 1], vec![0, 2, 5, 1]], vec![2, 5], vec![0, 0], 3), vec![vec![0, 1], vec![1, 1], vec![2, 1]]);
    assert_eq!(highest_ranked_k_items(vec![vec![1, 2, 0, 1], vec![1, 3, 3, 1], vec![0, 2, 5, 1]], vec![2, 3], vec![2, 3], 2), vec![vec![2, 1], vec![1, 2]]);
    assert_eq!(highest_ranked_k_items(vec![vec![1, 1, 1], vec![0, 0, 1], vec![2, 3, 4]], vec![2, 3], vec![0, 0], 3), vec![vec![2, 1], vec![2, 0]]);
}
