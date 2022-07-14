//! 2022春季编程大赛：守护太空城

pub fn defend_space_city(time: Vec<i32>, position: Vec<i32>) -> i32 {
    let mut pt = vec![vec![false; 5]; 105];
    let mut pos_len = 0;
    let mut time_len = 0;

    for (t, p) in time.into_iter().zip(position) {
        pt[p as usize][t as usize - 1] = true;
        pos_len = pos_len.max(p as usize + 1);
        time_len = time_len.max(t as usize);
    }
    let mut dp = vec![vec![-1; 1 << time_len]; pos_len + 2];
    dp[0][0] = 0;
    for i in 0..=pos_len {
        for s in 0..1 << time_len {
            if dp[i][s] == -1 {
                continue;
            }
            for j in 0..1 << time_len {
                let mut val = dp[i][s];
                let mut las1 = -100;
                let mut las2 = -100;
                for k in 0..time_len as i32 {
                    if j >> k & 1 == 1 {
                        val += 3.min(k - las2);
                        las2 = k;
                        continue;
                    }
                    if pt[i][k as usize] && s >> k & 1 == 0 {
                        val += 2.min(k - las1);
                        las1 = k;
                        continue;
                    }
                }
                if dp[i + 1][j] == -1 || dp[i + 1][j] > val {
                    dp[i + 1][j] = val;
                }
            }
        }
    }
    let mut result = i32::MAX;
    for i in 0..1 << time_len {
        if dp[pos_len + 1][i] != -1 {
            result = result.min(dp[pos_len + 1][i]);
        }
    }
    result
}

fn main() {
    assert_eq!(defend_space_city(vec![1], vec![100]), 2);
    assert_eq!(defend_space_city(vec![1, 1, 2, 5], vec![2, 3, 2, 3]), 6);
    assert_eq!(defend_space_city(vec![1, 1, 1, 2, 2, 3, 5], vec![1, 2, 3, 1, 2, 1, 3]), 9);
    assert_eq!(defend_space_city(vec![1, 2, 1], vec![6, 3, 3]), 5);
}
