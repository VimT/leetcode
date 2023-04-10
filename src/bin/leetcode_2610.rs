//! 转换二维数组

use std::collections::HashMap;

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *m.entry(num).or_default() += 1;
    }
    let len = m.values().max().unwrap().clone();
    let mut result = vec![vec![]; len as usize];
    for (num, times) in m {
        for i in 0..times {
            result[i as usize].push(num);
        }
    }
    result
}

fn main() {
    use leetcode::unorder_deep;
    fn test(func: fn(nums: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(unorder_deep(func(vec![1, 3, 4, 1, 2, 3, 1])), vec![vec![1], vec![1, 2, 3, 4], vec![1, 3]]);
        assert_eq!(unorder_deep(func(vec![1, 2, 3, 4])), vec![vec![1, 2, 3, 4]]);
    }
    test(find_matrix);
}
