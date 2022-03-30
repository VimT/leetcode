//! 森林中的兔子

use std::collections::HashMap;

pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut cnt = HashMap::new();
    for answer in answers {
        *cnt.entry(answer).or_insert(0i32) += 1;
    }
    for (k, v) in cnt {
        let num = k + 1;
        if v % num == 0 { result += v; } else {
            result += ((v / num) + 1) * num;
        }
    }
    result
}

fn main() {
    assert_eq!(num_rabbits(vec![1, 1, 2]), 5);
    assert_eq!(num_rabbits(vec![1, 0, 1, 0, 0]), 5);
    assert_eq!(num_rabbits(vec![10, 10, 10]), 11);
}
