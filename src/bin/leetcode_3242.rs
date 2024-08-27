//! 设计相邻元素求和服务

struct neighborSum {
    grid: Vec<Vec<i32>>,
    m: Vec<(usize, usize)>,
}

impl neighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let n = grid.len();
        let mut m = vec![(0, 0); n * n];
        for i in 0..n {
            for j in 0..n {
                m[grid[i][j] as usize] = (i, j);
            }
        }
        Self { grid, m }
    }
    fn find_value(&self, value: i32) -> (usize, usize) {
        self.m[value as usize]
    }
    fn adjacent_sum(&self, value: i32) -> i32 {
        let (i, j) = self.find_value(value);
        let n = self.grid.len();
        let mut sum = 0;
        if i > 0 {
            sum += self.grid[i - 1][j];
        }
        if i < n - 1 {
            sum += self.grid[i + 1][j];
        }
        if j > 0 {
            sum += self.grid[i][j - 1];
        }
        if j < n - 1 {
            sum += self.grid[i][j + 1];
        }
        sum
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let (i, j) = self.find_value(value);
        let n = self.grid.len();
        let mut sum = 0;
        if i > 0 && j > 0 {
            sum += self.grid[i - 1][j - 1];
        }
        if i > 0 && j < n - 1 {
            sum += self.grid[i - 1][j + 1];
        }
        if i < n - 1 && j > 0 {
            sum += self.grid[i + 1][j - 1];
        }
        if i < n - 1 && j < n - 1 {
            sum += self.grid[i + 1][j + 1];
        }
        sum
    }
}

fn main() {
    let n = neighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
    assert_eq!(n.adjacent_sum(1), 6);
    assert_eq!(n.adjacent_sum(4), 16);
    assert_eq!(n.diagonal_sum(4), 16);
    assert_eq!(n.diagonal_sum(8), 4);
}
