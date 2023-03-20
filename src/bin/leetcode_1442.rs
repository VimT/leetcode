//! 形成两个异或相等数组的三元组数目

use std::collections::HashMap;

pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut result = 0;
    let mut map = HashMap::with_capacity(len);
    let mut cur = 0;
    map.insert(0, (1, 0));
    for j in 0..len {
        cur ^= arr[j];
        let cnt = map.entry(cur).or_default();
        result += j * cnt.0 - cnt.1;
        cnt.0 += 1;
        cnt.1 += j + 1;
    }
    result as i32
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 3, 1, 6, 7]), 4);
        assert_eq!(func(vec![1, 1, 1, 1, 1]), 10);
        assert_eq!(func(vec![2, 3]), 0);
        assert_eq!(func(vec![1, 3, 5, 7, 9]), 3);
        assert_eq!(func(vec![7, 11, 12, 9, 5, 2, 7, 17, 22]), 8);
    }
    test(count_triplets);
}
