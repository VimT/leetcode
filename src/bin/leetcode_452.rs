//! 用最少数量的箭引爆气球

pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_unstable_by_key(|x| x[1]);
    let len = points.len();
    let mut result = 0;
    let mut i = 0;
    while i < len {
        let mut j = i + 1;
        while j < len && points[j][0] <= points[i][1] {
            j += 1;
        }
        result += 1;
        i = j;
    }
    result
}

fn main() {
    assert_eq!(find_min_arrow_shots(vec![vec![9, 12], vec![1, 10], vec![4, 11], vec![8, 12], vec![3, 9], vec![6, 9], vec![6, 7]]), 2);
    assert_eq!(find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]), 2);
    assert_eq!(find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]), 4);
    assert_eq!(find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]), 2);
}
