//! 找到两个数组的前缀公共数组

use std::collections::HashSet;

pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let len = a.len();
    let mut result = vec![0; len];
    let mut seta = HashSet::new();
    let mut setb = HashSet::new();

    for i in 0..len {
        seta.insert(a[i]);
        setb.insert(b[i]);
        result[i] = seta.intersection(&setb).count() as i32;
    }
    result
}

pub fn find_the_prefix_common_array2(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut x = 0u64;
    let mut y = 0;
    let mut result = Vec::with_capacity(a.len());
    for (a, b) in a.into_iter().zip(b) {
        x |= 1 << a;
        y |= 1 << b;
        result.push((x & y).count_ones() as i32)
    }
    result
}

fn main() {
    fn test(func: fn(a: Vec<i32>, b: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![1, 3, 2, 4], vec![3, 1, 2, 4]), vec![0, 2, 3, 4]);
        assert_eq!(func(vec![2, 3, 1], vec![3, 1, 2]), vec![0, 1, 3]);
    }
    test(find_the_prefix_common_array);
    test(find_the_prefix_common_array2);
}
