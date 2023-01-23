//! 统计参与通信的服务器

pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut result = 0;
    let mut row_sum = vec![0; m];
    let mut col_sum = vec![0; n];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                result += 1;
                row_sum[i] += 1;
                col_sum[j] += 1;
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 && row_sum[i] == 1 && col_sum[j] == 1 {
                result -= 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 0], vec![0, 1]]), 0);
        assert_eq!(func(vec![vec![1, 0], vec![1, 1]]), 3);
        assert_eq!(func(vec![vec![1, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]]), 4);
    }
    test(count_servers);
}
