//! 奇数值单元格的数目

pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut row = vec![0; m as usize];
    let mut col = vec![0; n as usize];
    for idx in &indices {
        row[idx[0] as usize] += 1;
        col[idx[1] as usize] += 1;
    }
    let mut result = 0;
    for rc in row {
        for &cc in &col {
            result += (rc + cc) & 1;
        }
    }
    result
}

pub fn odd_cells2(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut row = vec![0; m as usize];
    let mut col = vec![0; n as usize];
    for idx in &indices {
        row[idx[0] as usize] += 1;
        col[idx[1] as usize] += 1;
    }
    let odd_row: i32 = row.iter().map(|x| *x & 1).sum();
    let odd_col: i32 = col.iter().map(|x| *x & 1).sum();
    odd_row * (n - odd_col) + (m - odd_row) * odd_col
}

fn main() {
    fn test(func: fn(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
        assert_eq!(func(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
    }
    test(odd_cells);
    test(odd_cells2);
}
