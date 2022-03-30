//! 每个人戴不同帽子的方案数

pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
    const YU: i32 = 1e9 as i32 + 7;
    let len = hats.len();
    let mut max_hat = 0;
    for hat in &hats {
        for h in hat {
            max_hat = max_hat.max(*h as usize);
        }
    }
    let mut dp = vec![vec![0; 1 << len]; max_hat + 1];
    let mut hat_p = vec![vec![]; max_hat + 1];
    for people in 0..len {
        for &hat in &hats[people] {
            hat_p[hat as usize].push(people);
        }
    }
    dp[0][0] = 1;
    for i in 1..=max_hat {
        for j in 0..1 << len {
            let mut tmp = dp[i - 1][j];
            for &people in &hat_p[i] {
                if j & (1 << people) > 0 {
                    tmp += dp[i - 1][j & !(1 << people)];
                    tmp %= YU;
                }
            }
            dp[i][j] = tmp;
        }
    }
    dp[max_hat][(1 << len) - 1]
}

fn main() {
    assert_eq!(number_ways(vec![vec![3, 4], vec![4, 5], vec![5]]), 1);
    assert_eq!(number_ways(vec![vec![3, 5, 1], vec![3, 5]]), 4);
    assert_eq!(number_ways(vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![1, 2, 3, 4]]), 24);
    assert_eq!(number_ways(vec![vec![1, 2, 3], vec![2, 3, 5, 6], vec![1, 3, 7, 9], vec![1, 8, 9], vec![2, 5, 7]]), 111);
}
