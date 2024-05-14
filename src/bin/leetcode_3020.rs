//! 子集中元素的最大数量

use std::collections::BTreeMap;

pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let mut counter: BTreeMap<i32, i32> = BTreeMap::new();
    for x in nums {
        *counter.entry(x).or_insert(0) += 1;
    }
    let mut result = counter.remove(&1).map(|x| x - (x & 1 ^ 1)).unwrap_or(1);
    for (&k, &cnt) in &counter {
        if cnt <= 1 { continue; }
        let mut x = k;
        let mut len = 2;
        while let Some(nx) = x.checked_mul(x) {
            if let Some(&n) = counter.get(&nx) {
                len += 2;
                if n == 1 { break; }
                x = nx;
            } else {
                break;
            }
        }
        len -= 1;
        result = result.max(len);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![15, 15, 225, 225, 50625, 50625]), 5);
        assert_eq!(func(vec![5, 4, 1, 2, 2]), 3);
        assert_eq!(func(vec![1, 3, 2, 4]), 1);
    }
    test(maximum_length);
}
