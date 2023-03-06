//! 两个数组间的距离值

pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    arr1.into_iter().filter(|x| arr2.iter().all(|y| (x - y).abs() > d)).count() as _
}

fn main() {
    fn test(func: fn(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32) {
        assert_eq!(func(vec![4, 5, 8], vec![10, 9, 1, 8], 2), 2);
        assert_eq!(func(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3), 2);
        assert_eq!(func(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6), 1);
    }
    test(find_the_distance_value);
}
