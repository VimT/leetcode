//! 提取咒文

use std::collections::VecDeque;

/// 三维bfs
pub fn extract_mantra(matrix: Vec<String>, mantra: String) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let matrix: Vec<Vec<u8>> = matrix.into_iter().map(|x| x.into_bytes()).collect();
    let m = matrix.len();
    let n = matrix[0].len();
    let mantra = mantra.into_bytes();
    let len = mantra.len();
    let mut q = VecDeque::with_capacity(m * n);
    q.push_back((0, 0, 0, 1));
    let mut vis = vec![vec![vec![false; len]; n]; m]; // 标记访问过的位置
    vis[0][0][0] = true;
    while !q.is_empty() {
        let (x, y, k, step) = q.pop_front().unwrap();
        if matrix[x][y] == mantra[k] {
            if k == len - 1 { return step; }
            if !vis[x][y][k + 1] {
                vis[x][y][k + 1] = true;
                q.push_back((x, y, k + 1, step + 1));
            }
        } else {
            for (dx, dy) in DIR {
                let (nx, ny) = ((x as i32 + dx) as usize, ((y as i32 + dy) as usize));
                if nx < m && ny < n && !vis[nx][ny][k] {
                    vis[nx][ny][k] = true;
                    q.push_back((nx, ny, k, step + 1));
                }
            }
        }
    }
    -1
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(matrix: Vec<String>, mantra: String) -> i32) {
        assert_eq!(func(svec!["sd","ep"], String::from("speed")), 10);
        assert_eq!(func(svec!["abc","daf","geg"], String::from("sad")), -1);
    }
    test(extract_mantra);
}
