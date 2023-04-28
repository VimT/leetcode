//! 查询网格图中每一列的宽度

pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let m = grid.len();
    let n = grid[0].len();
    let mut result = vec![0; n];
    for i in 0..n {
        let mut max = 0;
        for j in 0..m {
            max = max.max(grid[j][i].to_string().len() as i32);
        }
        result[i] = max;
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1], vec![22], vec![333]]), vec![3]);
        assert_eq!(func(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]]), vec![3, 1, 2]);
    }
    test(find_column_width);
}
