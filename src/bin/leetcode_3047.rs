//! 求交集区域内的最大正方形面积

pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
    let len = bottom_left.len();
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            let (a1, b1) = (bottom_left[i][0], bottom_left[i][1]);
            let (c1, d1) = (top_right[i][0], top_right[i][1]);
            let (a2, b2) = (bottom_left[j][0], bottom_left[j][1]);
            let (c2, d2) = (top_right[j][0], top_right[j][1]);
            // 找两个矩形的交集
            let a = c1.min(c2) - a1.max(a2);
            let b = d1.min(d2) - b1.max(b2);
            if a > 0 && b > 0 {
                let side = a.min(b);
                result = result.max(side as i64 * side as i64);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(vec![vec![1, 1], vec![2, 2], vec![3, 1]], vec![vec![3, 3], vec![4, 4], vec![6, 6]]), 1);
        assert_eq!(func(vec![vec![1, 1], vec![2, 2], vec![1, 2]], vec![vec![3, 3], vec![4, 4], vec![3, 4]]), 1);
        assert_eq!(func(vec![vec![1, 1], vec![3, 3], vec![3, 1]], vec![vec![2, 2], vec![4, 4], vec![4, 2]]), 0);
    }
    test(largest_square_area);
}
