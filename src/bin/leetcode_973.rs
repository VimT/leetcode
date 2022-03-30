//! 最接近原点的 K 个点

pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    points.sort_unstable_by_key(|x| x[0] * x[0] + x[1] * x[1]);
    points[..k as usize].to_vec()
}

fn main() {
    assert_eq!(k_closest(vec![vec![1, 3], vec![-2, 2]], 1), vec![vec![-2, 2]]);
    assert_eq!(k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2), vec![vec![3, 3], vec![-2, 4]]);
}
