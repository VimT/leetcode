//! 查询后矩阵的和

pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut row_cnt = n as i64;
    let mut col_cnt = n as i64;
    let mut row_seen = vec![false; n];
    let mut col_seen = vec![false; n];
    let mut result = 0;
    for query in queries.into_iter().rev() {
        let (ty, idx, val) = (query[0], query[1] as usize, query[2] as i64);
        if ty == 0 {
            if !row_seen[idx] {
                row_seen[idx] = true;
                result += row_cnt * val;
                col_cnt -= 1;
            }
        } else {
            if !col_seen[idx] {
                col_seen[idx] = true;
                result += col_cnt * val;
                row_cnt -= 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, queries: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(3, vec![vec![0, 0, 1], vec![1, 2, 2], vec![0, 2, 3], vec![1, 0, 4]]), 23);
        assert_eq!(func(3, vec![vec![0, 0, 4], vec![0, 1, 2], vec![1, 0, 1], vec![0, 2, 3], vec![1, 2, 1]]), 17);
    }
    test(matrix_sum_queries);
}
