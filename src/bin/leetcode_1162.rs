//! 地图分析

use std::collections::{BinaryHeap, VecDeque};

static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

/// 朴素解法，超时
pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
    let mut all1 = true;
    let mut all0 = true;
    let len = grid.len();
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 1 { all0 = false; } else if grid[i][j] == 0 { all1 = false; }
        }
    }
    if all1 || all0 { return -1; }

    let mut result = -1;
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 0 {
                let mut seen = vec![vec![false; len]; len];
                let mut q = VecDeque::new();
                q.push_back((i, j));
                seen[i][j] = true;
                'out: while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    for (dx, dy) in DIR {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx >= 0 && nx < len as i32 && ny >= 0 && ny < len as i32 {
                            let (nx, ny) = (nx as usize, ny as usize);
                            if !seen[nx][ny] {
                                if grid[nx][ny] == 1 {
                                    result = result.max((nx as i32 - i as i32).abs() + (ny as i32 - j as i32).abs());
                                    break 'out;
                                } else {
                                    q.push_back((nx, ny));
                                    seen[nx][ny] = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    result
}


/// 多源最短路 的dijkstra做法，用陆地做源，看成单源
pub fn max_distance_dij(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut dp = vec![vec![i32::MAX; len]; len];
    let mut q = BinaryHeap::new();
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 1 {
                dp[i][j] = 0;
                q.push((0, i, j));
            }
        }
    }
    while !q.is_empty() {
        let (dis, x, y) = q.pop().unwrap();
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < len as i32 && ny >= 0 && ny < len as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if -dis + 1 < dp[nx][ny] {
                    dp[nx][ny] = -dis + 1;
                    q.push((dis - 1, nx, ny));
                }
            }
        }
    }
    let mut result = -1;
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 0 {
                result = result.max(dp[i][j]);
            }
        }
    }
    if result == i32::MAX { -1 } else { result }
}

/// dijkstra优化，因为边权都是1，不用优先队列，直接bfs也可以
pub fn max_distance_bfs(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut dp = vec![vec![i32::MAX; len]; len];
    let mut q = VecDeque::new();
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 1 {
                dp[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < len as i32 && ny >= 0 && ny < len as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if dp[x][y] + 1 < dp[nx][ny] {
                    dp[nx][ny] = dp[x][y] + 1;
                    q.push_back((nx, ny));
                }
            }
        }
    }
    let mut result = -1;
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 0 {
                result = result.max(dp[i][j]);
            }
        }
    }
    if result == i32::MAX { -1 } else { result }
}

/// 另一种bfs，从答案上看的，类似二叉树的层次遍历，从陆地按层遍历，看找到最远的海洋是多少层，关键是可以共用一个seen
pub fn max_distance_bfs2(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut seen = vec![vec![false; len]; len];
    let mut q = VecDeque::new();
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 1 {
                q.push_back((i, j));
                seen[i][j] = true;
            }
        }
    }
    if q.len() == 0 || q.len() == len * len {
        return -1;
    }
    let mut step = -1;
    while !q.is_empty() {
        step += 1;
        for _ in 0..q.len() {
            let (x, y) = q.pop_front().unwrap();
            for (dx, dy) in DIR {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < len as i32 && ny >= 0 && ny < len as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !seen[nx][ny] {
                        seen[nx][ny] = true;
                        q.push_back((nx, ny));
                    }
                }
            }
        }
    }
    step
}


/// 多源最短路的 spfa做法
pub fn max_distance_spfa(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut dp = vec![vec![i32::MAX; len]; len];
    let mut q = VecDeque::new();
    let mut inq = vec![vec![false; len]; len];
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 1 {
                dp[i][j] = 0;
                q.push_back((i, j));
                inq[i][j] = true;
            }
        }
    }

    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < len as i32 && ny >= 0 && ny < len as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if dp[nx][ny] > dp[x][y] + 1 {
                    dp[nx][ny] = dp[x][y] + 1;
                    if !inq[nx][ny] {
                        q.push_back((nx, ny));
                        inq[nx][ny] = true;
                    }
                }
            }
        }
    }

    let mut result = -1;
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 0 {
                result = result.max(dp[i][j]);
            }
        }
    }
    if result == i32::MAX { -1 } else { result }
}

/// 对于每个海洋区域 (x,y)，离它最近的陆地区域到它的路径要么从上方或者左方来，要么从右方或者下方来。
/// 两次动态规划，第一次从左上到右下，第二次从右下到左上，记f(x,y) 为(x,y) 距离最近的陆地区域的曼哈顿距离
pub fn max_distance_dp(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut dp = vec![vec![i32::MAX / 2; len]; len];
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 1 {
                dp[i][j] = 0;
            }
        }
    }
    // 从左上到右下
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 0 {
                if i >= 1 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
                }
                if j >= 1 {
                    dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
                }
            }
        }
    }
    // 从右下到左上
    for i in (0..len).rev() {
        for j in (0..len).rev() {
            if grid[i][j] == 0 {
                if i + 1 < len {
                    dp[i][j] = dp[i][j].min(dp[i + 1][j] + 1);
                }
                if j + 1 < len {
                    dp[i][j] = dp[i][j].min(dp[i][j + 1] + 1);
                }
            }
        }
    }
    let mut result = -1;
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == 0 {
                result = result.max(dp[i][j]);
            }
        }
    }
    if result >= i32::MAX / 2 { -1 } else { result }
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 0, 1, 1, 1], vec![0, 1, 1, 0, 0], vec![0, 0, 1, 1, 0], vec![1, 0, 0, 0, 0], vec![1, 1, 0, 0, 1]]), 2);
        assert_eq!(func(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]), 2);
        assert_eq!(func(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]), 4);
    }
    test(max_distance);
    test(max_distance_dij);
    test(max_distance_bfs);
    test(max_distance_bfs2);
    test(max_distance_spfa);
    test(max_distance_dp);
}
