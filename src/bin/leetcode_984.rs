//! 不含 AAA 或 BBB 的字符串

pub fn str_without3a3b(a: i32, b: i32) -> String {
    let (mut small_cnt, small_char) = if a > b { (b, b'b') } else { (a, b'a') };
    let (mut big_cnt, big_char) = if a <= b { (b, b'b') } else { (a, b'a') };

    let cnt = (big_cnt - small_cnt).min(small_cnt);
    let mut result = Vec::with_capacity((a + b) as usize);
    for _ in 0..cnt {
        result.push(big_char);
        result.push(big_char);
        result.push(small_char);
    }
    small_cnt -= cnt;
    big_cnt -= 2 * cnt;
    while small_cnt > 0 || big_cnt > 0 {
        if big_cnt > 0 {
            big_cnt -= 1;
            result.push(big_char);
        }
        if small_cnt > 0 {
            small_cnt -= 1;
            result.push(small_char);
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(str_without3a3b(1, 2), String::from("bba"));
    assert_eq!(str_without3a3b(4, 1), String::from("aabaa"));
}
