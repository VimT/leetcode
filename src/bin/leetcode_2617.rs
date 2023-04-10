//! 网格图中最少访问的格子数

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

/// 思路：对于一行或者一列来说，到达这一行的位置的步数，应该是线性递增的。
pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut q = VecDeque::with_capacity(m * n);
    q.push_back((0, 0));
    let mut rows = vec![vec![i32::MAX / 2; n]; m];
    let mut cols = vec![vec![i32::MAX / 2; m]; n];
    rows[0][0] = 1;
    cols[0][0] = 1;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        let step = rows[x][y].min(cols[y][x]);
        if (x, y) == (m - 1, n - 1) {
            return step;
        }
        let d = grid[x][y] as usize;
        let start = cols[y][x + 1..(x + d + 1).min(m)].binary_search_by(|num| num.cmp(&(step + 1)).then(Ordering::Less)).unwrap_err() + x + 1;
        for nx in start..(x + d + 1).min(m) {
            cols[y][nx] = step + 1;
            q.push_back((nx, y));
        }
        let start = rows[x][y + 1..(y + d + 1).min(n)].binary_search_by(|num| num.cmp(&(step + 1)).then(Ordering::Less)).unwrap_err() + y + 1;
        for ny in start..(y + d + 1).min(n) {
            rows[x][ny] = step + 1;
            q.push_back((x, ny));
        }
    }
    -1
}

/// 每一行存一个小顶堆，存(dis, j) 表示这一行 到达j列的dis，那么堆顶就是 当前位置的来源。
pub fn minimum_visited_cells2(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dis = vec![vec![i32::MAX >> 1; n]; m];
    let mut cols: Vec<BinaryHeap<(i32, usize)>> = vec![BinaryHeap::new(); n];
    dis[0][0] = 1;
    for i in 0..m {
        let mut row: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        for j in 0..n {
            while let Some(&(_, y)) = row.peek() {
                if y + grid[i][y] as usize >= j { break; }
                row.pop();
            }
            if let Some((_, y)) = row.peek() {
                dis[i][j] = dis[i][j].min(dis[i][*y] + 1);
            }

            while let Some(&(_, x)) = cols[j].peek() {
                if x + grid[x][j] as usize >= i { break; }
                cols[j].pop();
            }
            if let Some((_, x)) = cols[j].peek() {
                dis[i][j] = dis[i][j].min(dis[*x][j] + 1);
            }

            row.push((-dis[i][j], j));
            cols[j].push((-dis[i][j], i));
        }
    }
    if dis[m - 1][n - 1] >= i32::MAX >> 1 { -1 } else { dis[m - 1][n - 1] }
}

/// 每行/列 维护一个单调栈，每行单调栈 维护 (dis, j)
pub fn minimum_visited_cells3(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut col_st: Vec<Vec<(i32, i32)>> = vec![vec![]; n];
    let mut mn = i32::MAX;
    for i in (0..m).rev() {
        let mut st: Vec<(i32, i32)> = vec![];
        for j in (0..n).rev() {
            let st2 = &mut col_st[j];
            mn = i32::MAX;
            let g = grid[i][j];
            if (i, j) == (m - 1, n - 1) {
                mn = 0;
            } else if g > 0 {
                let k = st.binary_search_by(|x| x.1.cmp(&(-(j as i32 + g))).then(Ordering::Greater)).unwrap_err();
                if k < st.len() { mn = mn.min(st[k].0); }
                let k = st2.binary_search_by(|x| x.1.cmp(&(-(i as i32 + g))).then(Ordering::Greater)).unwrap_err();
                if k < st2.len() { mn = mn.min(st2[k].0); }
            }
            if mn == i32::MAX { continue; }
            mn += 1;
            while !st.is_empty() && mn <= st.last().unwrap().0 {
                st.pop();
            }
            while !st2.is_empty() && mn < st2.last().unwrap().0 {
                st2.pop();
            }
            st.push((mn, -(j as i32)));
            st2.push((mn, -(i as i32)));
        }
    }
    if mn == i32::MAX { -1 } else { mn }
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![19, 6, 0, 2], vec![4, 9, 6, 11], vec![19, 0, 14, 17], vec![2, 7, 4, 14], vec![8, 5, 16, 0]]), 3);
        assert_eq!(func(vec![vec![3, 4, 2, 1], vec![4, 2, 3, 1], vec![2, 1, 0, 0], vec![2, 4, 0, 0]]), 4);
        assert_eq!(func(vec![vec![3, 4, 2, 1], vec![4, 2, 1, 1], vec![2, 1, 1, 0], vec![3, 4, 1, 0]]), 3);
        assert_eq!(func(vec![vec![2, 1, 0], vec![1, 0, 0]]), -1);
    }
    test(minimum_visited_cells);
    test(minimum_visited_cells2);
    test(minimum_visited_cells3);
}
