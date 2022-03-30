//! 平均等待时间

pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut result = 0;
    let len = customers.len();
    let mut left = 0;
    for i in customers {
        let arrival = i[0] as i64;
        let need_time = i[1] as i64;
        if left == 0 || left < arrival {
            left = arrival;
        }
        result += left + need_time - arrival;
        left += need_time;
    }
    result as f64 / len as f64
}

fn main() {
    assert_eq!(average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]), 5.0);
    assert_eq!(average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]), 5.0);
    assert_eq!(average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]), 3.25);
}
