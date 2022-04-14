//! 招商银行-03. 点燃木棒

use std::collections::VecDeque;

/// 一个考耐心的模拟题
pub fn light_sticks(height: i32, width: i32, indices: Vec<i32>) -> Vec<i32> {
    let m = height as usize + 1;
    let n = width as usize + 1;
    // 表示每个节点的 四个方向能否访问
    let mut ava = vec![vec![[true; 4]; n]; m];
    const DIR: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let gun = n + n - 1;
    for idx in indices {
        let x = idx as usize / gun;
        let y = idx as usize % gun;
        if y < n - 1 {
            // 横的棍子
            ava[x][y][2] = false;
            ava[x][y + 1][0] = false;
        } else {
            // 竖的棍子
            let ry = y - (n - 1);
            ava[x][ry][3] = false;
            ava[x + 1][ry][1] = false;
        }
    }
    for i in 0..m {
        ava[i][0][0] = false;
        ava[i][n - 1][2] = false;
    }
    for i in 0..n {
        ava[0][i][1] = false;
        ava[m - 1][i][3] = false;
    }
    // 找到无法访问到的节点
    let mut ori_seen = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if !ava[i][j].iter().any(|x| *x) {
                ori_seen[i][j] = true;
            }
        }
    }
    let mut min_cost = i32::MAX;
    let mut result = vec![];
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if !ori_seen[i][j] {
                // 开始bfs
                let mut seen = ori_seen.clone();
                seen[i][j] = true;
                let mut cost = 0;
                q.push_back((i, j, 0));
                while !q.is_empty() {
                    let (x, y, cur) = q.pop_front().unwrap();
                    for i in 0..4 {
                        if ava[x][y][i] {
                            let (dx, dy) = DIR[i];
                            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                                let (nx, ny) = (nx as usize, ny as usize);
                                if !seen[nx][ny] {
                                    seen[nx][ny] = true;
                                    q.push_back((nx, ny, cur + 1));
                                    cost = cost.max(cur + 1);
                                }
                            }
                        }
                    }
                }
                // 检测有没访问到的
                for i in 0..m {
                    for j in 0..n {
                        if !seen[i][j] {
                            return vec![];
                        }
                    }
                }
                if cost == min_cost {
                    result.push((i * n + j) as i32);
                } else if cost < min_cost {
                    min_cost = cost;
                    result.clear();
                    result.push((i * n + j) as i32);
                }
            }
        }
    }

    result
}

fn main() {
    assert_eq!(light_sticks(1, 2, vec![3]), vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(light_sticks(2, 2, vec![2, 5, 6, 7, 8, 10, 11]), vec![2]);
    assert_eq!(light_sticks(1, 1, vec![0, 3]), vec![]);
}
