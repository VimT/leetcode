//! 反转之后不同整数的数目

use std::collections::HashSet;

pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
    let mut set: HashSet<i32> = nums.into_iter().collect();
    for mut num in set.iter().cloned().collect::<Vec<i32>>() {
        let mut new = 0;
        while num > 0 {
            new = new * 10 + num % 10;
            num /= 10;
        }
        set.insert(new);
    }
    set.len() as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 13, 10, 12, 31]), 6);
        assert_eq!(func(vec![2, 2, 2]), 1);
    }
    test(count_distinct_integers);
}
