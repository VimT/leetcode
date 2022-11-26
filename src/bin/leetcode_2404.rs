//! 出现最频繁的偶数元素

use std::collections::HashMap;

pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *map.entry(num).or_default() += 1;
    }
    let mut result = -1;
    let mut max = 0;
    for (num, cnt) in map {
        if num & 1 == 0 && (cnt > max || (cnt == max && num < result)) {
            result = num;
            max = cnt;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0, 1, 2, 2, 4, 4, 1]), 2);
        assert_eq!(func(vec![4, 4, 4, 9, 2, 4]), 4);
        assert_eq!(func(vec![29, 47, 21, 41, 13, 37, 25, 7]), -1);
    }
    test(most_frequent_even);
}
