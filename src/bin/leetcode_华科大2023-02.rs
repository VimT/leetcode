//! 特殊筷子

use std::cmp::Ordering;

pub fn special_chopsticks(mut chopsticks: Vec<i32>) -> i32 {
    chopsticks.sort_unstable();
    let len = chopsticks.len() as i32;
    for i in 1..=len {
        let x = chopsticks.binary_search_by(|num| num.cmp(&i).then(Ordering::Greater)).unwrap_err();
        if len - x as i32 == i {
            return i;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(chopsticks: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 5]), 2);
        assert_eq!(func(vec![0, 0]), -1);
        assert_eq!(func(vec![0, 4, 3, 0, 4]), 3);
        assert_eq!(func(vec![3, 6, 7, 7, 0]), -1);
    }
    test(special_chopsticks);
}
