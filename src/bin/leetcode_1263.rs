//! 推箱子

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

/// 朴素两层BFS
pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    let mut p = (0, 0);
    let mut b = (0, 0);
    let mut t = (0, 0);
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'B' { b = (i, j); } else if grid[i][j] == 'S' { p = (i, j); } else if grid[i][j] == 'T' { t = (i, j); }
        }
    }
    let mut q = VecDeque::new();
    q.push_back((b, p, 0));
    let mut seen = vec![vec![vec![vec![false; n]; m]; n]; m];
    while !q.is_empty() {
        let ((bx, by), (px, py), cur) = q.pop_front().unwrap();
        if (bx, by) == t {
            return cur;
        }
        for (dx, dy) in DIR {
            let (nbx, nby) = (bx as i32 + dx, by as i32 + dy);
            let (npx, npy) = (bx as i32 - dx, by as i32 - dy);
            if nbx >= 0 && nbx < m as i32 && nby >= 0 && nby < n as i32 && npx >= 0 && npx < m as i32 && npy >= 0 && npy < n as i32 {
                let (nbx, nby) = (nbx as usize, nby as usize);
                let (npx, npy) = (npx as usize, npy as usize);
                if grid[nbx][nby] != '#' && grid[npx][npy] != '#' && !seen[nbx][nby][bx][by] {
                    // 验证(px,py) -> (npx, npy);
                    let mut q2 = VecDeque::new();
                    let mut seen2 = vec![vec![false; n]; m];
                    q2.push_back((px, py));
                    seen2[bx][by] = true;
                    seen2[px][py] = true;
                    let mut ok = false;
                    while !q2.is_empty() {
                        let (px, py) = q2.pop_front().unwrap();
                        if (px, py) == (npx, npy) {
                            ok = true;
                            break;
                        }
                        for (dx, dy) in DIR {
                            let (nx, ny) = (px as i32 + dx, py as i32 + dy);
                            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                                let (nx, ny) = (nx as usize, ny as usize);
                                if grid[nx][ny] != '#' && !seen2[nx][ny] {
                                    seen2[nx][ny] = true;
                                    q2.push_back((nx, ny));
                                }
                            }
                        }
                    }
                    if ok {
                        q.push_back(((nbx, nby), (bx, by), cur + 1));
                        seen[nbx][nby][bx][by] = true;
                    }
                }
            }
        }
    }
    -1
}


fn dis(a: (usize, usize), b: (usize, usize), s: i32) -> i32 {
    let x = (a.0 as i32 - b.0 as i32).abs();
    let y = (a.1 as i32 - b.1 as i32).abs();
    return x + y + s;
}


/// 箱子用了A*，估价函数f返回值里，第一比较值是取曼哈顿距离和已走步数之和，保证在箱体内搜索出步数最优值，后续比较值是box的坐标。
/// 人用best-first算法，原理和A*相似，只是不用考虑最短步数，只优先考虑距离终点近点的扩展。
pub fn min_push_box2(grid: Vec<Vec<char>>) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    let mut p = (0, 0);
    let mut b = (0, 0);
    let mut t = (0, 0);
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'B' { b = (i, j); } else if grid[i][j] == 'S' { p = (i, j); } else if grid[i][j] == 'T' { t = (i, j); }
        }
    }
    let mut q = BinaryHeap::new();
    q.push(Reverse((dis(b, t, 0), 0, b, p)));
    let mut seen = vec![vec![vec![vec![false; n]; m]; n]; m];
    while !q.is_empty() {
        let Reverse((_, step, (bx, by), (px, py))) = q.pop().unwrap();
        if (bx, by) == t {
            return step;
        }
        for (dx, dy) in DIR {
            let (nbx, nby) = (bx as i32 + dx, by as i32 + dy);
            let (npx, npy) = (bx as i32 - dx, by as i32 - dy);
            if nbx >= 0 && nbx < m as i32 && nby >= 0 && nby < n as i32 && npx >= 0 && npx < m as i32 && npy >= 0 && npy < n as i32 {
                let (nbx, nby) = (nbx as usize, nby as usize);
                let (npx, npy) = (npx as usize, npy as usize);
                if grid[nbx][nby] != '#' && grid[npx][npy] != '#' && !seen[nbx][nby][bx][by] {
                    let mut ok = false;    // 验证(px,py) -> (npx, npy);
                    let mut q2 = BinaryHeap::new();
                    let mut seen2 = vec![vec![false; n]; m];
                    q2.push(Reverse((dis((px, py), (npx, npy), 0), px, py)));
                    seen2[bx][by] = true;
                    seen2[px][py] = true;
                    while !q2.is_empty() {
                        let Reverse((_, x, y)) = q2.pop().unwrap();
                        if (x, y) == (npx, npy) {
                            ok = true;
                            break;
                        }
                        for (dx, dy) in DIR {
                            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                                let (nx, ny) = (nx as usize, ny as usize);
                                if grid[nx][ny] != '#' && !seen2[nx][ny] {
                                    q2.push(Reverse((dis((nx, ny), (npx, npy), 0), nx, ny)));
                                    seen2[nx][ny] = true;
                                }
                            }
                        }
                    }

                    if ok {
                        q.push(Reverse((dis((nbx, nby), t, step + 1), step + 1, (nbx, nby), (bx, by))));
                        seen[nbx][nby][bx][by] = true;
                    }
                }
            }
        }
    }
    -1
}


fn main() {
    fn test(func: fn(grid: Vec<Vec<char>>) -> i32) {
        assert_eq!(func(vec![vec!['#', '#', '#', '#', '#', '#'],
                             vec!['#', 'T', '#', '#', '#', '#'],
                             vec!['#', '.', '.', 'B', '.', '#'],
                             vec!['#', '.', '#', '#', '.', '#'],
                             vec!['#', '.', '.', '.', 'S', '#'],
                             vec!['#', '#', '#', '#', '#', '#']]), 3);
        assert_eq!(func(vec![
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#'],
            vec!['#', '.', '.', '.', '#', '#', '#', '#', '.', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '#', 'T', '#', '.', '.', '#', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '#', '#', '.', '#'],
            vec!['#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '#', '#', '.', '#'],
            vec!['#', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', 'B', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'S', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#']]
        ), 26);
        assert_eq!(func(vec![vec!['#', '#', '.', '#', '.', '.', '.', '.'],
                             vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                             vec!['.', '.', '.', '.', '.', 'T', '.', '#'],
                             vec!['.', '.', '.', '.', '.', '#', '.', '.'],
                             vec!['.', '.', '.', '.', '.', '#', '.', '.'],
                             vec!['.', '.', '.', '.', '.', '.', 'S', '.'],
                             vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                             vec!['.', '.', '.', '.', '.', '.', '.', '.']]), 6);
        assert_eq!(func(vec![vec!['#', '#', '#', '#', '#', '#'],
                             vec!['#', 'T', '#', '#', '#', '#'],
                             vec!['#', '.', '.', 'B', '.', '#'],
                             vec!['#', '#', '#', '#', '.', '#'],
                             vec!['#', '.', '.', '.', 'S', '#'],
                             vec!['#', '#', '#', '#', '#', '#']]), -1);
        assert_eq!(func(vec![vec!['#', '#', '#', '#', '#', '#'],
                             vec!['#', 'T', '.', '.', '#', '#'],
                             vec!['#', '.', '#', 'B', '.', '#'],
                             vec!['#', '.', '.', '.', '.', '#'],
                             vec!['#', '.', '.', '.', 'S', '#'],
                             vec!['#', '#', '#', '#', '#', '#']]), 5);
        assert_eq!(func(vec![vec!['#', '#', '#', '#', '#', '#', '#'],
                             vec!['#', 'S', '#', '.', 'B', 'T', '#'],
                             vec!['#', '#', '#', '#', '#', '#', '#']]), -1);
    }
    test(min_push_box);
    test(min_push_box2);
}
