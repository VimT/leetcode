//! 奇怪的打印机 II

use std::collections::VecDeque;

pub fn is_printable(grid: Vec<Vec<i32>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let max_color = *grid.iter().map(|x| x.iter().max().unwrap()).max().unwrap() as usize + 1;
    let mut range = vec![(m, 0, n, 0); max_color];
    for i in 0..m {
        for j in 0..n {
            let color = grid[i][j] as usize;
            range[color].0 = range[color].0.min(i);
            range[color].1 = range[color].1.max(i);
            range[color].2 = range[color].2.min(j);
            range[color].3 = range[color].3.max(j);
        }
    }
    let mut g = vec![vec![false; max_color]; max_color];
    let mut indegree = vec![0; max_color];
    let mut cnt = 0;
    for (color, (mnx, mxx, mny, mxy)) in range.into_iter().enumerate() {
        if mnx != m {
            cnt += 1;
            for i in mnx..=mxx {
                for j in mny..=mxy {
                    if grid[i][j] as usize != color {
                        if !g[color][grid[i][j] as usize] {
                            g[color][grid[i][j] as usize] = true;
                            indegree[grid[i][j] as usize] += 1;
                        }
                    }
                }
            }
        } else { indegree[color] = -1; }
    }

    let mut q = VecDeque::new();
    for color in 0..max_color {
        if indegree[color] == 0 {
            cnt -= 1;
            q.push_back(color);
        }
    }
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for v in 0..max_color {
            if g[u][v] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    cnt -= 1;
                    q.push_back(v);
                }
            }
        }
    }
    cnt == 0
}

fn main() {
    fn test(func: fn(target_grid: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![1, 1, 1, 1], vec![1, 2, 2, 1], vec![1, 2, 2, 1], vec![1, 1, 1, 1]]), true);
        assert_eq!(func(vec![vec![1, 1, 1, 1], vec![1, 1, 3, 3], vec![1, 1, 3, 4], vec![5, 5, 1, 4]]), true);
        assert_eq!(func(vec![vec![1, 2, 1], vec![2, 1, 2], vec![1, 2, 1]]), false);
        assert_eq!(func(vec![vec![1, 1, 1], vec![3, 1, 3]]), false);
    }
    test(is_printable);
}
