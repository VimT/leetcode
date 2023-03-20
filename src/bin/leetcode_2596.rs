//! 检查骑士巡视方案

pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
    let n = grid.len();
    let mut arr = vec![(0, 0); n * n];
    for i in 0..n {
        for j in 0..n {
            arr[grid[i][j] as usize] = (i, j);
        }
    }
    if arr[0] != (0, 0) {
        return false;
    }
    for i in 0..n * n - 1 {
        let da = (arr[i].0 as i32 - arr[i + 1].0 as i32).abs();
        let db = (arr[i].1 as i32 - arr[i + 1].1 as i32).abs();
        if da * da + db * db != 5 {
            return false;
        }
    }
    true
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![24, 11, 22, 17, 4], vec![21, 16, 5, 12, 9], vec![6, 23, 10, 3, 18], vec![15, 20, 1, 8, 13], vec![0, 7, 14, 19, 2]]), false);
        assert_eq!(func(vec![vec![0, 11, 16, 5, 20], vec![17, 4, 19, 10, 15], vec![12, 1, 8, 21, 6], vec![3, 18, 23, 14, 9], vec![24, 13, 2, 7, 22]]), true);
        assert_eq!(func(vec![vec![0, 3, 6], vec![5, 8, 1], vec![2, 7, 4]]), false);
    }
    test(check_valid_grid);
}
