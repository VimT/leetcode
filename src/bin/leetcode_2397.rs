//! 被列覆盖的最多行数

pub fn maximum_rows(mat: Vec<Vec<i32>>, cols: i32) -> i32 {
    let n = mat[0].len();
    let rows: Vec<i32> = mat.iter().map(|x| {
        let mut row = 0;
        for i in 0..n {
            if x[i] == 1 {
                row |= 1 << i;
            }
        }
        row
    }).collect();

    let mut result = 0;
    for i in 0..1i32 << n {
        if i.count_ones() == cols as u32 {
            result = result.max(rows.iter().filter(|x| **x & i == **x).count() as i32);
        }
    }
    result
}

fn main() {
    assert_eq!(maximum_rows(vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]], 2), 3);
    assert_eq!(maximum_rows(vec![vec![1], vec![0]], 1), 2);
}
