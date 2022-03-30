//! 可以形成最大正方形的矩形数目

pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let mut max_len = 0;
    let mut result = 0;
    for rec in rectangles {
        let bian = rec[0].min(rec[1]);
        if bian > max_len {
            max_len = bian;
            result = 1;
        } else if bian == max_len {
            result += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]]), 3);
    assert_eq!(count_good_rectangles(vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]]), 3);
}
