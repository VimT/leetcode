//! 两个最好的不重叠活动

pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
    let len = events.len();
    events.sort_unstable_by_key(|x| x[0]);
    let mut max = vec![0; len + 1];
    let mut cur_max = 0;
    for i in (0..len).rev() {
        cur_max = cur_max.max(events[i][2]);
        max[i] = cur_max;
    }
    let mut result = 0;
    for i in 0..len {
        let next_start = events.binary_search_by_key(&(events[i][1] + 1), |x| x[0]).unwrap_or_else(|x| x);
        result = result.max(events[i][2] + max[next_start]);
    }
    result
}

fn main() {
    assert_eq!(max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]]), 4);
    assert_eq!(max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]]), 5);
    assert_eq!(max_two_events(vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]]), 8);
}