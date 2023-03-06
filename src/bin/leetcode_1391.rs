//! 检查网格中是否存在有效路径


use std::collections::VecDeque;

pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let left = (0, -1);
    let right = (0, 1);
    let up = (-1, 0);
    let down = (1, 0);
    let dir = |num| {
        match num {
            1 => [left, right],
            2 => [up, down],
            3 => [left, down],
            4 => [right, down],
            5 => [left, up],
            6 => [right, up],
            _ => unreachable!(),
        }
    };
    let mut q = VecDeque::new();
    q.push_back((0,0));
    let mut seen = vec![vec![false;n];m];
    seen[0][0] = true;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        if (x, y) == (m - 1, n - 1) {
            return true;
        }
        for (dx, dy) in dir(grid[x][y]) {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if nx < m && ny < n && !seen[nx][ny] {
                let d = dir(grid[nx][ny]);
                if d[0] == (-dx, -dy) || d[1] == (-dx, -dy) {
                    q.push_back((nx, ny));
                    seen[nx][ny] = true;
                }
            }
        }
    }
    false
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![2, 4, 3], vec![6, 5, 2]]), true);
        assert_eq!(func(vec![vec![1, 2, 1], vec![1, 2, 1]]), false);
        assert_eq!(func(vec![vec![1, 1, 2]]), false);
    }
    test(has_valid_path);
}
