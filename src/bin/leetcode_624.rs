//! 数组列表中的最大距离

pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut pre_max = *arrays[0].last().unwrap();
    let mut pre_min = arrays[0][0];
    let mut result = 0;
    for array in &arrays[1..] {
        result = result.max((array[0] - pre_max).abs());
        result = result.max((*array.last().unwrap() - pre_min).abs());
        pre_max = pre_max.max(*array.last().unwrap());
        pre_min = pre_min.min(array[0]);
    }
    result
}

fn main() {
    fn test(func: fn(arrays: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]), 4);
        assert_eq!(func(vec![vec![1], vec![1]]), 0);
    }
    test(max_distance);
}
