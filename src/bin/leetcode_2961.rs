//! 双模幂运算

use leetcode::algorithm::quick_pow;

pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
    variables.into_iter().enumerate().filter_map(|(i, v)| {
        let (a, b, c, d) = (v[0] as i64, v[1] as i64, v[2] as i64, v[3] as i64);
        let x = quick_pow(a, b, 10);
        if quick_pow(x, c, d) == target as i64 {
            Some(i as i32)
        } else { None }
    }).collect()
}

fn main() {
    fn test(func: fn(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32>) {
        assert_eq!(func(vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]], 2), vec![0, 2]);
        assert_eq!(func(vec![vec![39, 3, 1000, 1000]], 17), vec![]);
    }
    test(get_good_indices);
}
