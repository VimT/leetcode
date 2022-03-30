//! 猜数字大小 II

static mut DP: Option<[[usize; 202]; 202]> = None;

pub fn get_money_amount(n: i32) -> i32 {
    unsafe {
        if DP.is_none() {
            let n = 200;
            let mut dp = [[0; 202]; 202];
            for i in (1..n).rev() {
                for j in i + 1..=n {
                    let mut tmp = usize::MAX;
                    for k in i..=j {
                        tmp = tmp.min(k + dp[i][k - 1].max(dp[k + 1][j]));
                    }
                    dp[i][j] = tmp;
                }
            }
            DP = Some(dp);
        }
        DP.as_ref().unwrap()[1][n as usize] as i32
    }
}

fn main() {
    assert_eq!(get_money_amount(10), 16);
    assert_eq!(get_money_amount(1), 0);
    assert_eq!(get_money_amount(2), 1);
}
