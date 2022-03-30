//! 完成比赛的最少时间

///每种胎可以换任意次，预处理出跑x圈的最小时间，然后dp[i]表示跑i圈需要的最小时间
/// dp[i] = dp[i-j] + min_sec[j] + change_time for j in 1..18
pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
    let mut min_sec = vec![i32::MAX / 2; 18];
    for tire in &tires {
        let (f, r) = (tire[0], tire[1]);
        let mut i = 1;
        let mut sum = 0;
        let mut time = f;
        while time <= change_time + f {
            sum += time;
            min_sec[i] = min_sec[i].min(sum);
            time *= r;
            i += 1;
        }
    }
    let mut dp = vec![i32::MAX / 2; num_laps as usize + 1];
    dp[0] = -change_time;
    for i in 1..=num_laps as usize {
        for j in 1..=i.min(17) {
            dp[i] = dp[i].min(dp[i - j] + min_sec[j] + change_time);
        }
    }
    dp[num_laps as usize]
}

fn main() {
    assert_eq!(minimum_finish_time(vec![vec![2, 3], vec![3, 4]], 5, 4), 21);
    assert_eq!(minimum_finish_time(vec![vec![1, 10], vec![2, 2], vec![3, 4]], 6, 5), 25);
}
