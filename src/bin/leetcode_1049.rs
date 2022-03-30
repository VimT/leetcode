//! 最后一块石头的重量 II

/// dp[i+1][j] 表示前 i 个石头能否凑出重量 j
pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let len = stones.len();
    let sum = stones.iter().sum::<i32>() as usize;
    let half_sum = sum / 2;
    let mut dp = vec![vec![false; half_sum + 1]; len + 1];
    dp[0][0] = true;
    for i in 1..=len {
        for j in 0..=half_sum {
            if stones[i - 1] > j as i32 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - stones[i - 1] as usize];
            }
        }
    }
    for i in (0..=half_sum).rev() {
        if dp[len][i] {
            return (sum - 2 * i) as i32;
        }
    }
    0
}

fn main() {
    assert_eq!(last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    assert_eq!(last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
    assert_eq!(last_stone_weight_ii(vec![1, 2]), 1);
    assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
}

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut q = std::collections::BinaryHeap::new();
    for i in stones {
        q.push(i);
    }
    while q.len() > 1 {
        let num1 = q.pop().unwrap();
        let num2 = q.pop().unwrap();
        let num = num1 - num2;
        if num > 0 {
            q.push(num);
        }
    }
    q.pop().unwrap_or(0)
}