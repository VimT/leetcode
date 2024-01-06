//! 传送卷轴

use std::collections::VecDeque;

/// 二分
pub fn challenge_of_the_keeper(maze: Vec<String>) -> i32 {
    let len = maze.len();
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let maze: Vec<Vec<u8>> = maze.into_iter().map(|x| x.into_bytes()).collect();
    let (mut sx, mut sy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    for i in 0..len {
        for j in 0..len {
            if maze[i][j] == b'S' {
                sx = i;
                sy = j;
            } else if maze[i][j] == b'T' {
                tx = i;
                ty = j;
            }
        }
    }
    // 算终点到其他位置的距离
    let mut q = VecDeque::new();
    q.push_back((tx, ty));
    let mut dis = vec![vec![i32::MAX; len]; len];
    dis[tx][ty] = 0;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (dx, dy) in DIR {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if nx < len && ny < len && maze[nx][ny] != b'#' && dis[nx][ny] > dis[x][y] + 1 {
                dis[nx][ny] = dis[x][y] + 1;
                q.push_back((nx, ny));
            }
        }
    }
    let mut left = 0;
    let mut right = (len * len + 1) as i32;
    struct DFS {
        maze: Vec<Vec<u8>>,
        max_dis: i32,
        dis: Vec<Vec<i32>>,
        len: usize,
    }
    impl DFS {
        fn dfs(&mut self, i: usize, j: usize, vis: &mut Vec<Vec<bool>>) -> bool {
            if vis[i][j] || self.maze[i][j] == b'#' {
                return false;
            }
            if self.maze[i][j] == b'T' {
                return true;
            }
            vis[i][j] = true;

            // 释放卷轴
            if self.maze[i][j] == b'.' {
                let (x, y) = (i, self.len - 1 - j);
                if self.maze[x][y] != b'#' && self.dis[x][y] > self.max_dis {
                    return false;
                }
                let (x, y) = (self.len - 1 - i, j);
                if self.maze[x][y] != b'#' && self.dis[x][y] > self.max_dis {
                    return false;
                }
            }

            for (dx, dy) in DIR {
                let (nx, ny) = ((i as i32 + dx) as usize, (j as i32 + dy) as usize);
                if nx < self.len && ny < self.len && self.dfs(nx, ny, vis) { return true; }
            }
            false
        }
    }
    let mut dfs = DFS { maze, max_dis: 0, dis, len };
    while left < right {
        let mid = (left + right) / 2;
        dfs.max_dis = mid;
        if dfs.dfs(sx, sy, &mut vec![vec![false; len]; len]) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left > (len * len) as i32 { -1 } else { left }
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(maze: Vec<String>) -> i32) {
        assert_eq!(func(svec![".....","##S..","...#.","T.#..","###.."]), 7);
        assert_eq!(func(svec![".#..","..##",".#S.",".#.T"]), -1);
        assert_eq!(func(svec!["S###.","..###","#..##","##..#","###.T"]), 5);
    }
    test(challenge_of_the_keeper);
}
