//! 统计一个圆中点的数目

pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    #[inline(always)]
    fn sqr(x: i32) -> i32 { x * x }
    queries.into_iter().map(|q| {
        let (x, y, r) = (q[0], q[1], sqr(q[2]));
        points.iter().filter(|p| {
            let (x1, y1) = (p[0], p[1]);
            sqr(x1 - x) + sqr(y1 - y) <= r
        }).count() as i32
    }).collect()
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]], vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]]), vec![3, 2, 2]);
        assert_eq!(func(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]], vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]]), vec![2, 3, 2, 4]);
    }
    test(count_points);
}
