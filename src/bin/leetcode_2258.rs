//! 逃离火灾


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Fire(i32),
    Wall,
}

impl Tile {
    fn can_pass(self, time: i32, is_end: bool) -> bool {
        match self {
            Empty => true,
            Fire(fire_time) => {
                if is_end { fire_time >= time } else { fire_time > time }
            }
            Wall => false,
        }
    }
}

use std::collections::VecDeque;
use Tile::*;

/// bfs + 二分查找
pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
    const MAX: i32 = 1e9 as i32;
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    let mut g = vec![vec![Empty; n]; m];
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            g[i][j] = match grid[i][j] {
                0 => Empty,
                1 => {
                    q.push_back((i, j, 0));
                    Fire(0)
                }
                2 => Wall,
                _ => unreachable!(),
            }
        }
    }
    while !q.is_empty() {
        let (x, y, dis) = q.pop_front().unwrap();
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if g[nx][ny] == Empty {
                    g[nx][ny] = Fire(dis + 1);
                    q.push_back((nx, ny, dis + 1));
                }
            }
        }
    }

    let mut check = |start_time: i32| -> bool {
        if !g[0][0].can_pass(start_time, false) { return false; }
        q.clear();
        let mut vis = vec![vec![false; n]; m];
        vis[0][0] = true;
        q.push_back((0, 0, start_time));
        while !q.is_empty() {
            let (x, y, time) = q.pop_front().unwrap();
            if x == m - 1 && y == n - 1 { return true; }
            for (dx, dy) in DIR {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !vis[nx][ny] && g[nx][ny].can_pass(time + 1, nx == m - 1 && ny == n - 1) {
                        vis[nx][ny] = true;
                        q.push_back((nx, ny, time + 1));
                    }
                }
            }
        }
        false
    };
    // 先走一次看看能不能过去
    if !check(0) {
        return -1;
    }
    if check(MAX) {
        return MAX;
    }

    let mut left = 0;
    let mut right = (m * n) as i32 + 1;
    while left < right {
        let mid = (left + right) >> 1;
        if check(mid) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left - 1
}

/// 两次bfs，火距离屋子的距离，比人距离屋子的距离长多少，就可以等多久
pub fn maximum_minutes2(grid: Vec<Vec<i32>>) -> i32 {
    const MAX: i32 = 1e9 as i32;
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::new();
    let mut solve = |ex: usize, ey: usize| {
        if grid[ex][ey] == 2 { return -1; };
        let mut people = MAX;
        let mut fire = MAX;
        let mut vis = vec![vec![false; n]; m];
        q.push_back((ex, ey, 0));
        vis[ex][ey] = true;
        while !q.is_empty() {
            let (x, y, dis) = q.pop_front().unwrap();
            if fire == MAX && grid[x][y] == 1 {
                fire = dis;
            }
            if people == MAX && x == 0 && y == 0 {
                people = dis;
            }
            for (dx, dy) in DIR {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if matches!(grid[nx][ny], 0 | 1) && !vis[nx][ny] {
                        vis[nx][ny] = true;
                        q.push_back((nx, ny, dis + 1));
                    }
                }
            }
        }
        if people == MAX { return -1; }
        if fire == MAX { return MAX; }
        return fire - people;
    };

    let mut result = solve(m - 1, n - 1);//粗算时间
    if result == -1 || result == MAX { return result; }
    // 如果人与火会同时到达左和上，意味着会烧到屁股，需要少等1分钟。
    let tg = solve(m - 2, n - 1).max(solve(m - 1, n - 2));
    if result == tg { result -= 1; }
    result.max(-1)
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 2, 0, 0, 0, 0, 0], vec![0, 0, 0, 2, 2, 1, 0], vec![0, 2, 0, 0, 1, 2, 0], vec![0, 0, 2, 2, 2, 0, 2], vec![0, 0, 0, 0, 0, 0, 0]]), 3);
        assert_eq!(func(vec![vec![0, 0, 0, 0], vec![0, 1, 2, 0], vec![0, 2, 0, 0]]), -1);
        assert_eq!(func(vec![vec![0, 0, 0], vec![2, 2, 0], vec![1, 2, 0]]), 1000000000);
    }
    test(maximum_minutes);
    test(maximum_minutes2);
}
