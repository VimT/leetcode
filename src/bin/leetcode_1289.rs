//! 下降路径最小和  II

pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut min1 = 0;
    let mut min2 = 0;
    let mut min_index = len;
    for i in 0..len {
        let mut m1 = i32::MAX;
        let mut m2 = i32::MAX;
        let mut mi = len;
        for j in 0..len {
            let n = if min_index == j { min2 } else { min1 } + grid[i][j];
            if n < m1 {
                m2 = m1;
                m1 = n;
                mi = j;
            } else if n < m2 {
                m2 = n;
            }
        }
        min1 = m1;
        min2 = m2;
        min_index = mi;
    }
    min1
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![2, 2, 1, 2, 2], vec![2, 2, 1, 2, 2], vec![2, 2, 1, 2, 2], vec![2, 2, 1, 2, 2], vec![2, 2, 1, 2, 2]]), 7);
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), 13);
        assert_eq!(func(vec![vec![7]]), 7);
    }
    test(min_falling_path_sum);
}
