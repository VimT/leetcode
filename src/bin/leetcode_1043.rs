//! 分隔数组以得到最大和

pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let len = arr.len();
    let k = k as usize;
    let mut dp = vec![0; len + 1];
    for i in 1..=len {
        let mut max = 0;
        for pre in 0..k.min(i) {
            max = max.max(arr[i - 1 - pre]);
            dp[i] = dp[i].max(dp[i - pre - 1] + max * (pre + 1) as i32);
        }
    }
    dp[len]
}

fn main() {
    assert_eq!(max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3), 84);
    assert_eq!(max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4), 83);
    assert_eq!(max_sum_after_partitioning(vec![1], 1), 1);
}
