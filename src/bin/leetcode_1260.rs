//! 二维网格迁移


pub fn shift_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut g2 = Vec::with_capacity(m * n);
    for line in &grid {
        g2.extend_from_slice(line);
    }
    let k = k as usize % (m * n);
    g2.reverse();
    g2[..k].reverse();
    g2[k..].reverse();
    let mut iter = g2.into_iter();
    for i in 0..m {
        for j in 0..n {
            grid[i][j] = iter.next().unwrap();
        }
    }
    grid
}

pub fn shift_grid2(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let k = k as usize;
    let size = n * m;
    let mut result = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let nxy = (i * n + j + k) % size;
            let x = nxy / n;
            let y = nxy % n;
            result[x][y] = grid[i][j];
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![1]], 100), [[1]]);
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1), vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        assert_eq!(func(vec![vec![3, 8, 1, 9], vec![19, 7, 2, 5], vec![4, 6, 11, 10], vec![12, 0, 21, 13]], 4), vec![vec![12, 0, 21, 13], vec![3, 8, 1, 9], vec![19, 7, 2, 5], vec![4, 6, 11, 10]]);
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9), vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }
    test(shift_grid);
    test(shift_grid2);
}
