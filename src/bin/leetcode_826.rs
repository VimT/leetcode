//! 安排工作以达到最大收益

pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let mut dp: Vec<(i32, i32)> = difficulty.into_iter().zip(profit).collect();
    dp.sort_unstable_by_key(|x| (x.0, -x.1));
    let len = dp.len();
    let mut cur_max = 0;
    for i in 0..len {
        cur_max = cur_max.max(dp[i].1);
        dp[i].1 = cur_max;
    }
    let mut result = 0;
    for worker in worker {
        match dp.binary_search_by_key(&worker, |x| x.0) {
            Ok(v) => { result += dp[v].1; }
            Err(v) => { if v > 0 { result += dp[v - 1].1; } }
        }
    }
    result
}

fn main() {
    assert_eq!(max_profit_assignment(vec![23, 30, 35, 35, 43, 46, 47, 81, 83, 98], vec![8, 11, 11, 20, 33, 37, 60, 72, 87, 95], vec![95, 46, 47, 97, 11, 35, 99, 56, 41, 92]), 553);
    assert_eq!(max_profit_assignment(vec![2, 4, 6, 8, 10], vec![10, 20, 30, 40, 50], vec![4, 5, 6, 7]), 100);
    assert_eq!(max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25]), 0);
}
