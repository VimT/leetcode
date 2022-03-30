//! 面试题 16.19. 水域大小

use std::collections::VecDeque;

pub fn pond_sizes(land: Vec<Vec<i32>>) -> Vec<i32> {
    let m = land.len();
    let n = land[0].len();
    let mut result = vec![];
    let mut q = VecDeque::new();
    let mut vis = vec![vec![false; n]; m];
    let dir = [-1, 0, 1, 0, -1, 1, 1, -1, -1];
    for i in 0..m {
        for j in 0..n {
            if land[i][j] == 0 && !vis[i][j] {
                q.push_back((i, j));
                let mut cnt = 0;
                vis[i][j] = true;
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    cnt += 1;
                    for i in 0..8 {
                        let nx = x as i32 + dir[i];
                        let ny = y as i32 + dir[i + 1];
                        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && land[nx as usize][ny as usize] == 0 && !vis[nx as usize][ny as usize] {
                            let (nx, ny) = (nx as usize, ny as usize);
                            vis[nx][ny] = true;
                            q.push_back((nx, ny));
                        }
                    }
                }
                result.push(cnt);
            }
        }
    }
    result.sort_unstable();
    result
}

fn main() {
    assert_eq!(pond_sizes(vec![
        vec![0, 2, 1, 0],
        vec![0, 1, 0, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 0, 1],
    ]), vec![1, 2, 4])
}
