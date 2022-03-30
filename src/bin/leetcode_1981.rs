//! 最小化目标值与所选元素的差


pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
    let len = mat.len();
    let mut dp = vec![false; 1];
    dp[0] = true;
    let mut curmax = 0;
    for i in 0..len {
        let max = *mat[i].iter().max().unwrap() as usize;
        let mut new_dp = vec![false; curmax + max + 1];
        for &num in &mat[i] {
            for j in num as usize..=curmax + num as usize {
                new_dp[j] |= dp[j - num as usize];
            }
        }
        dp = new_dp;
        curmax += max;
    }
    let mut result = 4900;
    for i in len..=curmax {
        if dp[i] {
            result = result.min((i as i32 - target).abs());
        }
    }

    result
}

fn main() {
    assert_eq!(minimize_the_difference(vec![vec![1], vec![2], vec![3]], 100), 94);
    assert_eq!(minimize_the_difference(vec![vec![1, 2, 9, 8, 7]], 6), 1);
    assert_eq!(minimize_the_difference(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13), 0);
}

