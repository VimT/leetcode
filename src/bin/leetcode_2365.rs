//! 任务调度器 II

use std::collections::HashMap;

pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
    let mut last_run = HashMap::new();
    let mut cur = 0;
    for task in tasks {
        cur = cur.max(last_run.get(&task).cloned().unwrap_or(i64::MIN) + space as i64) + 1;
        last_run.insert(task, cur);
    }
    cur
}

fn main() {
    assert_eq!(task_scheduler_ii(vec![1, 2, 1, 2, 3, 1], 3), 9);
    assert_eq!(task_scheduler_ii(vec![5, 8, 8, 5], 2), 6);
}
