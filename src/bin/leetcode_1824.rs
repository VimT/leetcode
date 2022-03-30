//! 最少侧跳次数

pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
    let len = obstacles.len();
    let mut dp = [1, 0, 1];
    for i in 1..len {
        let mut new_dp = [0, 0, 0];
        for j in 0..3 {
            if obstacles[i] == j + 1 {
                continue;
            }
            if obstacles[i - 1] == j + 1 {
                let mut tmp = i32::MAX;
                for k in 0..3 {
                    if k != j && obstacles[i] != k + 1 && obstacles[i - 1] != k + 1 {
                        tmp = tmp.min(dp[k as usize] + 1);
                    }
                }
                new_dp[j as usize] = tmp;
            } else {
                new_dp[j as usize] = dp[j as usize];
            }
        }
        dp = new_dp;
    }
    *dp.iter().min().unwrap()
}


fn main() {
    assert_eq!(min_side_jumps(vec![0, 0, 3, 1, 0, 1, 0, 2, 3, 1, 0]), 2);
    assert_eq!(min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
    assert_eq!(min_side_jumps(vec![0, 1, 1, 3, 3, 0]), 0);
    assert_eq!(min_side_jumps(vec![0, 2, 1, 0, 3, 0]), 2);
}
