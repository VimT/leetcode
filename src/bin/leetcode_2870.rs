//! 使数组为空的最少操作次数

use std::collections::HashMap;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for num in nums { *cnt.entry(num).or_default() += 1; }
    let mut result = 0;
    for (_, cnt) in cnt {
        if cnt == 1 { return -1; }
        result += (cnt + 2) / 3; // 向上取整 upper(a/b) = (a+b-1)/b
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]), 4);
        assert_eq!(func(vec![2, 1, 2, 2, 3, 3]), -1);
    }
    test(min_operations);
}
