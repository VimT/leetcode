//! 解决智力问题

pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let len = questions.len();
    let mut dp = vec![0; len + 1];
    for (i, q) in questions.iter().enumerate().rev() {
        let next_available = 1 + i + q[1] as usize;
        dp[i] = dp[i + 1].max(q[0] as i64 + if (next_available) < len { dp[next_available] } else { 0 });
    }
    dp[0]
}

fn main() {
    assert_eq!(most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]), 5);
    assert_eq!(most_points(vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]]), 7);
}
