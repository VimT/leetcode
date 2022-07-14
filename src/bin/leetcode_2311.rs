//! 小于等于 K 的最长二进制子序列

pub fn longest_subsequence(s: String, k: i32) -> i32 {
    let mut cur = 0;
    let mut result = 0;
    for (i, &ch) in s.as_bytes().iter().rev().enumerate() {
        if ch == b'1' {
            if i < 31 && cur | 1 << i <= k {
                result += 1;
                cur |= 1 << i;
            }
        } else {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("100110111111000000010011101000111011000001000111010001010111100001111110110010100011100100111000011011000000100001011000000100110110001101011010011"), 522399436), 92);
        assert_eq!(func(String::from("1001010"), 5), 5);
        assert_eq!(func(String::from("00101001"), 1), 6);
    }
    test(longest_subsequence);
}
