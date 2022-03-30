//! 最长连续序列

use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let s: HashSet<i32> = nums.into_iter().collect();
    let mut ret = 0;
    for i in s.iter() {
        let mut i = *i;
        if s.contains(&(i - 1)) { continue; };
        let mut tmp = 0;
        while s.contains(&i) {
            tmp += 1;
            i += 1;
        };
        ret = ret.max(tmp);
    }
    ret
}

fn main() {
    assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
}
