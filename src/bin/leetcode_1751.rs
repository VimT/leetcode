//! 最多可以参加的会议数目 II

pub fn max_value_recursive(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
    fn recursive(events: &Vec<Vec<i32>>, idx: usize, last_end_time: i32, left_k: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if left_k == 0 || idx == events.len() {
            return 0;
        }
        if cache[idx][left_k] > 0 {
            return cache[idx][left_k];
        }
        let mut result = 0;
        if events[idx][0] > last_end_time {
            let value = events[idx][2];
            let end_time = events[idx][1];
            let mut next_idx = events.binary_search(&vec![end_time + 1, 0, 0]).unwrap_or_else(|x| x);
            while next_idx < events.len() && events[next_idx][0] <= end_time {
                next_idx += 1;
            }
            result = result.max(recursive(events, next_idx, end_time, left_k - 1, cache)) + value;
        }
        result = result.max(recursive(events, idx + 1, last_end_time, left_k, cache));
        cache[idx][left_k] = result;
        result
    }
    events.sort_unstable_by_key(|x| x[0]);
    let mut cache = vec![vec![0; k as usize + 1]; events.len()];
    recursive(&events, 0, 0, k as usize, &mut cache)
}


/// dp[i][j] 到第i件会议结束已经参加了k次会议的最大会议价值
pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
    events.sort_unstable_by_key(|x| x[1]);
    let len = events.len();
    let k = k as usize;
    let mut dp = vec![vec![0; k + 1]; len + 1];
    // dp[1][1] = dp[0][1] .max (
    for i in 1..=len {
        let start_time = events[i - 1][0];
        let value = events[i - 1][2];
        let mut end = events.binary_search_by_key(&start_time, |x| x[1]).unwrap_or_else(|x| x);
        while end > 0 && events[end - 1][1] >= start_time {
            end -= 1;
        }
        for j in 1..=k {
            dp[i][j] = dp[i - 1][j].max(dp[end][j - 1] + value);
        }
    }
    dp[len][k]
}


fn main() {
    assert_eq!(max_value(vec![vec![62, 69, 80], vec![35, 38, 48], vec![22, 85, 41], vec![13, 30, 43], vec![25, 82, 23], vec![13, 57, 40], vec![5, 67, 1], vec![1, 6, 84], vec![72, 88, 23], vec![29, 32, 48], vec![51, 83, 50], vec![31, 81, 57], vec![56, 94, 38], vec![12, 65, 44], vec![6, 13, 24], vec![8, 53, 1], vec![21, 51, 56], vec![88, 95, 6], vec![29, 72, 48], vec![44, 66, 45], vec![17, 88, 1]], 7), 283);
    assert_eq!(max_value(vec![vec![11, 17, 56], vec![24, 40, 53], vec![5, 62, 67], vec![66, 69, 84], vec![56, 89, 15]], 2), 151);
    assert_eq!(max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2), 7);
    assert_eq!(max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]], 2), 10);
    assert_eq!(max_value(vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]], 3), 9);
}
