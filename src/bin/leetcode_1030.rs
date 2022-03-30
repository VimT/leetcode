//! 距离顺序排列矩阵单元格

pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::with_capacity((rows * cols) as usize);
    for i in 0..rows {
        for j in 0..cols {
            result.push(vec![i, j]);
        }
    }
    result.sort_unstable_by_key(|x| (x[0] - r_center).abs() + (x[1] - c_center).abs());
    result
}

fn main() {
    assert_eq!(all_cells_dist_order(1, 2, 0, 0), vec![vec![0, 0], vec![0, 1]]);
    assert_eq!(all_cells_dist_order(2, 2, 0, 1), vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]]);
    assert_eq!(all_cells_dist_order(2, 3, 1, 2), vec![vec![1, 2], vec![0, 2], vec![1, 1], vec![0, 1], vec![1, 0], vec![0, 0]]);
}
