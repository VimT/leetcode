//! 数组的相对排序

/// 桶排序
pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut m = vec![0; 1001];
    for &num in arr1.iter() {
        m[num as usize] += 1
    }
    let mut result = Vec::with_capacity(arr1.len());
    for num in arr2 {
        for _ in 0..m[num as usize] {
            result.push(num);
        }
        m[num as usize] = 0;
    }
    for i in 0..=1000 {
        for _ in 0..m[i as usize] {
            result.push(i);
        }
    }
    result
}

fn main() {
    fn test(func: fn(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], vec![2, 1, 4, 3, 9, 6]), vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]);
        assert_eq!(func(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]), vec![22, 28, 8, 6, 17, 44]);
    }
    test(relative_sort_array);
}
