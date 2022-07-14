//! 最大休假天数

pub fn max_vacation_days(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
    if days.len() == 0 || flights.len() == 0 { return 0; };
    let mut dp = vec![vec![0; days[0].len() + 1]; days.len()];
    for week in (0..days[0].len()).rev() {
        for cur_city in 0..days.len() {
            dp[cur_city][week] = days[cur_city][week] + dp[cur_city][week + 1];
            for dest_city in 0..days.len() {
                if flights[cur_city][dest_city] == 1 {
                    dp[cur_city][week] = dp[cur_city][week].max(days[dest_city][week] + dp[dest_city][week + 1]);
                }
            }
        }
    }
    return dp[0][0];
}

fn main() {
    fn test(func: fn(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]], vec![vec![1, 3, 1], vec![6, 0, 3], vec![3, 3, 3]]), 12);
        assert_eq!(func(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], vec![vec![1, 1, 1], vec![7, 7, 7], vec![7, 7, 7]]), 3);
        assert_eq!(func(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]], vec![vec![7, 0, 0], vec![0, 7, 0], vec![0, 0, 7]]), 21);
    }
    test(max_vacation_days);
}
