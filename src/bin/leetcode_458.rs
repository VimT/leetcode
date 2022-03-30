//! 可怜的小猪

pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    if buckets == 1 { return 0; }
    let buckets = buckets as usize;
    let mut combinations = vec![vec![0; buckets + 1]; buckets + 1];
    combinations[0][0] = 1;
    let iter = (minutes_to_test / minutes_to_die) as usize;
    let mut dp = vec![vec![0; iter + 1]; buckets];
    for i in 0..buckets {
        dp[i][0] = 1;
    }
    for j in 0..=iter {
        dp[0][j] = 1;
    }
    for i in 1..buckets {
        combinations[i][0] = 1;
        combinations[i][i] = 1;
        for j in 1..i {
            combinations[i][j] = combinations[i - 1][j - 1] + combinations[i - 1][j];
        }
        for j in 1..=iter {
            for k in 0..=i {
                dp[i][j] += dp[k][j - 1] * combinations[i][i - k];
            }
        }
        if dp[i][iter] >= buckets {
            return i as i32;
        }
    }
    0
}

fn main() {
    assert_eq!(poor_pigs(1000, 15, 60), 5);
    assert_eq!(poor_pigs(4, 15, 15), 2);
    assert_eq!(poor_pigs(4, 15, 30), 2);
}

