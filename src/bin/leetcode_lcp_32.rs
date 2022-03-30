//! LCP 32. 批量处理任务

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// https://leetcode-cn.com/problems/t3fKg1/solution/10xing-jie-jue-zhan-dou-by-foxtail-ke2e/
/// 对于一堆都已经开始的任务，当系统运行a时长后，所有任务的最晚启动时刻也会集体后移a（或完成）；
/// amazing!!
pub fn process_tasks(mut tasks: Vec<Vec<i32>>) -> i32 {
    tasks.push(vec![1e9 as i32 + 1, 1e9 as i32 + 1, 1]);
    let mut has_run_time = 0;
    let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    tasks.sort_unstable();
    for task in tasks {
        let (start, end, period) = (task[0], task[1], task[2]);
        while !heap.is_empty() && heap.peek().unwrap().0.0 + has_run_time < start {
            let Reverse((last_start_time, end_time)) = *heap.peek().unwrap();
            // there + has_run_time 得到入队到出队这段时间运行了多久
            if last_start_time + has_run_time >= end_time {
                heap.pop().unwrap();
            } else {
                has_run_time += end_time.min(start) - (last_start_time + has_run_time);
            }
        }
        // there - has_run_time
        heap.push(Reverse((end - period + 1 - has_run_time, end + 1)));
    }
    has_run_time
}

pub fn process_tasks_baoli(mut tasks: Vec<Vec<i32>>) -> i32 {
    tasks.sort_unstable_by_key(|x| (x[1], x[0]));
    let mut ranges: Vec<(i32, i32)> = vec![];
    for task in tasks {
        let (start, end, mut period) = (task[0], task[1], task[2]);
        if ranges.is_empty() || ranges.last().unwrap().1 < start {
            ranges.push((end - period + 1, end));
        } else {
            let mut i = ranges.len() - 1;
            while period > 0 {
                let (left, right) = ranges[i];
                if left >= start {
                    period -= right - left + 1;
                } else { break; }
                if i == 0 { break; }
                i -= 1;
            }
            if period > 0 {
                let (left, right) = ranges[i];
                if right >= start && left < start {
                    period -= right - start + 1;
                }
            }
            if period > 0 {
                let mut now = end - period + 1;
                while !ranges.is_empty() && now <= ranges.last().unwrap().1 {
                    let (left, right) = ranges.pop().unwrap();
                    now = left - (right - now + 1);
                }
                ranges.push((now, end));
            }
        }
    }
    let mut result = 0;
    for (left, right) in ranges {
        result += right - left + 1;
    }
    result
}

fn main() {
    assert_eq!(process_tasks_baoli(vec![vec![10, 42, 6], vec![47, 81, 35], vec![38, 58, 13], vec![39, 48, 4], vec![12, 62, 24], vec![54, 73, 1], vec![59, 96, 34], vec![65, 91, 20]]), 54);
    assert_eq!(process_tasks_baoli(vec![vec![1, 3, 2], vec![2, 5, 3], vec![5, 6, 2]]), 4);
    assert_eq!(process_tasks_baoli(vec![vec![2, 3, 1], vec![5, 5, 1], vec![5, 6, 2]]), 3);
}
