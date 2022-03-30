//! 统计隐藏数组数目

pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let mut min = 0;
    let mut max = 0;
    let mut cur = 0;
    for diff in differences {
        cur -= diff as i64;
        min = min.min(cur);
        max = max.max(cur);
    }
    (upper as i64 - max - lower as i64 + min + 1).max(0) as i32
}

fn main() {
    assert_eq!(number_of_arrays(vec![1, -3, 4], 1, 6), 2);
    assert_eq!(number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5), 4);
    assert_eq!(number_of_arrays(vec![4, -7, 2], 3, 6), 0);
}
