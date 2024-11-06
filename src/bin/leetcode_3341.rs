//! 到达最后一个房间的最少时间 I

use std::collections::BinaryHeap;

pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let m = move_time.len();
    let n = move_time[0].len();
    let mut q = BinaryHeap::new();
    q.push((0, 0, 0));
    let mut dis = vec![vec![i32::MAX; n]; m];
    dis[0][0] = 0;
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while let Some((_, x, y)) = q.pop() {
        if (x, y) == (m - 1, n - 1) {
            return dis[x][y];
        }
        for (dx, dy) in DIR {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx < m && ny < n && dis[nx][ny] > (dis[x][y] + 1).max(move_time[nx][ny] + 1) {
                dis[nx][ny] = (dis[x][y] + 1).max(move_time[nx][ny] + 1);
                q.push((-dis[nx][ny], nx, ny));
            }
        }
    }
    unreachable!()
}

fn main() {
    use leetcode::vvec;
    fn test(func: fn(move_time: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vvec![[15,58],[67,4]]), 60);
        assert_eq!(func(vvec![[0,4],[4,4]]), 6);
        assert_eq!(func(vvec![[0,0,0],[0,0,0]]), 3);
        assert_eq!(func(vvec![[0,1],[1,2]]), 3);
    }
    test(min_time_to_reach);
}
