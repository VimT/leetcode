//! 总持续时间可被 60 整除的歌曲

pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut cnt = vec![0i64; 60];
    for num in time {
        cnt[(num % 60) as usize] += 1;
    }
    let mut result = cnt[0] * (cnt[0] - 1) / 2;
    result += cnt[30] * (cnt[30] - 1) / 2;
    for i in 1..30 {
        result += cnt[i] * cnt[60 - i];
    }
    result as i32
}

fn main() {
    assert_eq!(num_pairs_divisible_by60(vec![60; 60000]), 1799970000);
    assert_eq!(num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]), 3);
    assert_eq!(num_pairs_divisible_by60(vec![60, 60, 60]), 3);
}
