//! 使循环数组所有元素相等的最少秒数

use std::collections::HashMap;

pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, Vec<usize>> = HashMap::new();
    let len = nums.len();
    for (i, num) in nums.into_iter().enumerate() {
        cnt.entry(num).or_default().push(i);
    }
    cnt.into_iter().map(|(_, map)| {
        let mut result = (map[0] + len - *map.last().unwrap()) / 2;
        for diff in map.windows(2) {
            result = result.max((diff[1] - diff[0]) / 2);
        }
        result
    }).min().unwrap() as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 11, 11, 11, 19, 12, 8, 7, 19]), 2);
        assert_eq!(func(vec![8, 8, 9, 10, 9]), 1);
        assert_eq!(func(vec![2, 1, 3, 3, 2]), 2);
        assert_eq!(func(vec![1, 2, 1, 2]), 1);
        assert_eq!(func(vec![5, 5, 5, 5]), 0);
    }
    test(minimum_seconds);
}
