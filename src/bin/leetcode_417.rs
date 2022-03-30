//! 太平洋大西洋水流问题

use std::collections::VecDeque;

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let m = heights.len();
    let n = heights[0].len();
    let mut set = vec![vec![[false, false]; n]; m];
    let mut q = VecDeque::new();
    let dir = [-1, 0, 1, 0, -1];
    for i in 0..m {
        q.push_back((i, 0, 0));
        q.push_back((i, n - 1, 1));
    }
    for j in 0..n {
        q.push_back((0, j, 0));
        q.push_back((m - 1, j, 1));
    }

    while !q.is_empty() {
        let (x, y, sea) = q.pop_front().unwrap();
        if set[x][y][sea] == true { continue; }
        set[x][y][sea] = true;
        for i in 0..4 {
            let (dx, dy) = (dir[i], dir[i + 1]);
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && ny >= 0 && nx < m as i32 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if !set[nx][ny][sea] && heights[nx][ny] >= heights[x][y] {
                    q.push_back((nx, ny, sea));
                }
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if set[i][j][0] && set[i][j][1] {
                result.push(vec![i as i32, j as i32]);
            }
        }
    }
    result
}

fn main() {
    assert_eq!(pacific_atlantic(vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]), vec![vec![0, 2], vec![1, 0], vec![1, 1], vec![1, 2], vec![2, 0], vec![2, 1], vec![2, 2]]);
    assert_eq!(pacific_atlantic(vec![vec![1, 2, 2, 3, 5], vec![3, 2, 3, 4, 4], vec![2, 4, 5, 3, 1], vec![6, 7, 1, 4, 5], vec![5, 1, 1, 2, 4]]), vec![vec![0, 4], vec![1, 3], vec![1, 4], vec![2, 2], vec![3, 0], vec![3, 1], vec![4, 0]]);
    assert_eq!(pacific_atlantic(vec![vec![2, 1], vec![1, 2]]), vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]);
}
