//! 穿越网格图的安全路径

use std::collections::VecDeque;

/// 0-1 BFS：对 Dijkstra 算法的优化。
/// 因为边权只有 0 和 1，我们可以把最小堆换成双端队列，遇到 0 边权就加入队首，遇到 1 边权就加入队尾，这样可以保证队首总是最小的，就不需要最小堆了。
pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let mut dis = vec![vec![i32::MAX / 2; n]; m];
    dis[0][0] = grid[0][0];
    while let Some((x, y)) = q.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx < m && ny < n && dis[x][y] + grid[nx][ny] < dis[nx][ny] {
                dis[nx][ny] = dis[x][y] + grid[nx][ny];
                if grid[nx][ny] == 0 {
                    q.push_front((nx, ny));
                } else {
                    q.push_back((nx, ny));
                }
            }
        }
    }
    dis[m - 1][n - 1] < health
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>, health: i32) -> bool) {
        assert_eq!(func(vec![vec![1, 0, 1, 1], vec![0, 0, 0, 1], vec![1, 0, 1, 1], vec![0, 1, 1, 0], vec![1, 0, 0, 1]], 4), true);
        assert_eq!(func(vec![vec![0, 1, 0, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 0, 0, 1, 0]], 1), true);
        assert_eq!(func(vec![vec![0, 1, 1, 0, 0, 0], vec![1, 0, 1, 0, 0, 0], vec![0, 1, 1, 1, 0, 1], vec![0, 0, 1, 0, 1, 0]], 3), false);
        assert_eq!(func(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]], 5), true);
    }
    test(find_safe_walk);
}
