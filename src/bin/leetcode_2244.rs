//! 完成所有任务需要的最少轮数

use std::collections::HashMap;

pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let mut cnt = HashMap::new();
    for task in tasks {
        *cnt.entry(task).or_insert(0) += 1;
    }
    let mut result = 0;
    for (_, num) in cnt {
        if num == 1 {
            return -1;
        }
        result += num / 3;
        if num % 3 != 0 {
            result += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]), 4);
    assert_eq!(minimum_rounds(vec![2, 3, 3]), -1);
}
