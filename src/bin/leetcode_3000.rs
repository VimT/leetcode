//! 对角线最长的矩形的面积

pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
    dimensions.into_iter().map(|x| {
        (x[0] * x[0] + x[1] * x[1], x[0] * x[1])
    }).max().unwrap().1
}

fn main() {
    fn test(func: fn(dimensions: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![9, 3], vec![8, 6]]), 48);
        assert_eq!(func(vec![vec![3, 4], vec![4, 3]]), 12);
    }
    test(area_of_max_diagonal);
}
