//! 矩阵中战斗力最弱的 K 行


pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut lines: Vec<(i32, i32)> = mat.into_iter().map(|x| x.partition_point(|&x| x == 1) as i32).zip(0..).collect();
    lines.sort_unstable();
    lines.into_iter().take(k as usize).map(|x| x.1).collect()
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 1, 0, 0, 0],
                             vec![1, 1, 1, 1, 0],
                             vec![1, 0, 0, 0, 0],
                             vec![1, 1, 0, 0, 0],
                             vec![1, 1, 1, 1, 1]], 3), vec![2, 0, 3]);
        assert_eq!(func(vec![vec![1, 0, 0, 0],
                             vec![1, 1, 1, 1],
                             vec![1, 0, 0, 0],
                             vec![1, 0, 0, 0]], 2), vec![0, 2]);
    }
    test(k_weakest_rows);
}
