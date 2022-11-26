//! 处理用时最长的那个任务的员工

pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
    let mut cost = vec![0; n as usize];
    cost[logs[0][0] as usize] = logs[0][1];
    for i in 1..logs.len() {
        let len = logs[i][1] - logs[i - 1][1];
        cost[logs[i][0] as usize] = cost[logs[i][0] as usize].max(len);
    }
    let mut max = 0;
    let mut result = 0;
    for i in 0..n as usize {
        if cost[i] > max {
            max = cost[i];
            result = i;
        }
    }
    result as i32
}

fn main() {
    assert_eq!(hardest_worker(10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]), 1);
    assert_eq!(hardest_worker(26, vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]]), 3);
    assert_eq!(hardest_worker(2, vec![vec![0, 10], vec![1, 20]]), 0);
}
