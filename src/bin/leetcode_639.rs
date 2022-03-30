//! 解码方法 II

pub fn num_decodings(s: String) -> i32 {
    let yu = 1e9 as u64 + 7;
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![1_u64; len + 1];
    fn count2(first_char: u8, second_char: u8) -> u64 {
        match first_char {
            b'0' => 0,
            b'*' => {
                match second_char {
                    b'0'..=b'6' => 2,
                    b'*' => 15,
                    _ => 1,
                }
            }
            b'1' => {
                match second_char {
                    b'*' => 9,
                    _ => 1
                }
            }
            b'2' => {
                match second_char {
                    b'0'..=b'6' => 1,
                    b'*' => 6,
                    _ => 0,
                }
            }
            _ => 0
        }
    }
    fn count1(char: u8) -> u64 {
        match char {
            b'0' => 0,
            b'*' => 9,
            _ => 1
        }
    }
    for i in 1..=len {
        let a = dp[i - 1] * count1(s[i - 1]) % yu;
        let b = if i > 1 { dp[i - 2] * count2(s[i - 2], s[i - 1]) } else { 0 } % yu;
        dp[i] = (a + b) % yu;
    }
    dp[len] as i32
}

fn main() {
    assert_eq!(num_decodings(String::from("*")), 9);
    assert_eq!(num_decodings(String::from("1*")), 18);
    assert_eq!(num_decodings(String::from("2*")), 15);
}
