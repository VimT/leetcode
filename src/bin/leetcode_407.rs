//! 接雨水 II

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    let m = height_map.len();
    let n = height_map[0].len();
    if m <= 2 || n <= 2 {
        return 0;
    }
    let mut q = BinaryHeap::new();
    let mut visited = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                q.push(Reverse((height_map[i][j], i, j)));
                visited[i][j] = true;
            }
        }
    }
    let mut res = 0;
    let dirs = [-1, 0, 1, 0, -1];
    while !q.is_empty() {
        let Reverse((cur_h, x, y)) = q.pop().unwrap();
        for k in 0..4 {
            let nx = x as i32 + dirs[k];
            let ny = y as i32 + dirs[k + 1];
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && !visited[nx as usize][ny as usize] {
                let (nx, ny) = (nx as usize, ny as usize);
                if height_map[nx][ny] < cur_h {
                    res += cur_h - height_map[nx][ny];
                }
                visited[nx][ny] = true;
                q.push(Reverse((height_map[nx][ny].max(cur_h), nx, ny)));
            }
        }
    }
    res
}

fn main() {
    assert_eq!(trap_rain_water(vec![vec![3, 3, 3, 3, 3], vec![3, 2, 2, 2, 3], vec![3, 2, 1, 2, 3], vec![3, 2, 2, 2, 3], vec![3, 3, 3, 3, 3]]), 10);
    assert_eq!(trap_rain_water(vec![vec![1, 4, 3, 1, 3, 2], vec![3, 2, 1, 3, 2, 4], vec![2, 3, 3, 2, 3, 1]]), 4);
}
