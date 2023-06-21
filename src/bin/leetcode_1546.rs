//! 和为目标值且不重叠的非空子数组的最大数目


use std::collections::HashSet;

pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
    let mut set = HashSet::new();
    let mut sum = 0;
    set.insert(0);
    let mut result = 0;
    for num in nums {
        sum += num;
        if set.contains(&(sum - target)) {
            result += 1;
            sum = 0;
            set.clear();
        }
        set.insert(sum);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![1, 1, 1, 1, 1], 2), 2);
        assert_eq!(func(vec![-1, 3, 5, 1, 4, 2, -9], 6), 2);
        assert_eq!(func(vec![-2, 6, 6, 3, 5, 4, 1, 2, 8], 10), 3);
        assert_eq!(func(vec![0, 0, 0], 0), 3);
    }
    test(max_non_overlapping);
}
