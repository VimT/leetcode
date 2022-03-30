//! 自由之路

pub fn find_rotate_steps(ring: String, key: String) -> i32 {
    let ring = ring.as_bytes();
    let key = key.as_bytes();
    let rlen = ring.len();
    let klen = key.len();
    let mut dp = vec![i32::MAX; rlen];
    let mut m = vec![vec![]; 26];
    for i in 0..rlen {
        m[(ring[i] - b'a') as usize].push(i);
    }
    dp[0] = 0;
    let start = vec![0];
    for i in 0..klen {
        for &target_pos in &m[(key[i] - b'a') as usize] {
            let mut min = i32::MAX;
            let last = if i > 0 { &m[(key[i - 1] - b'a') as usize] } else { &start };
            for &last_pos in last {
                let zhuan = ((target_pos + rlen - last_pos) % rlen).min((last_pos + rlen - target_pos) % rlen);
                min = min.min(zhuan as i32 + dp[last_pos] + 1);
            }
            dp[target_pos] = min;
        }
    }

    let mut result = i32::MAX;
    for &pos in &m[(key[klen - 1] - b'a') as usize] {
        result = result.min(dp[pos]);
    }
    result
}

fn main() {
    assert_eq!(find_rotate_steps(String::from("godding"), String::from("gd")), 4);
    assert_eq!(find_rotate_steps(String::from("godding"), String::from("godding")), 13);
}
