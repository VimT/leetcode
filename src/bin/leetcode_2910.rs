//! 合法分组的最少组数

use std::collections::HashMap;

pub fn min_groups_for_valid_assignment(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let len = nums.len();
    for num in nums {
        *cnt.entry(num).or_default() += 1;
    }
    let cnt: Vec<_> = cnt.values().copied().collect();
    for k in (2..=len as i32 + 1).rev() {
        let mut can = true;
        let mut result = 0;
        for &num in &cnt {
            let a = (num + k - 1) / k;
            let b = a * k - num;
            if b > a {
                can = false;
                break;
            }
            result += a;
        }
        if can {
            return result;
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 1, 1, 1, 1]), 1);
        assert_eq!(func(vec![3, 1, 3, 1, 3, 3, 2, 2, 1, 1, 2, 2, 1]), 3);
        assert_eq!(func(vec![1, 2]), 2);
        assert_eq!(func(vec![1, 1]), 1);
        assert_eq!(func(vec![10, 10, 10, 3, 1, 1]), 4);
        assert_eq!(func(vec![1, 1, 3, 3, 1, 1, 2, 2, 3, 1, 3, 2]), 5);
        assert_eq!(func(vec![3, 2, 3, 2, 3]), 2);
    }
    test(min_groups_for_valid_assignment);
}
