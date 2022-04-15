//! 按字典序排在最后的子串

/// 双指针
pub fn last_substring(s: String) -> String {
    let s = s.as_bytes();
    let mut i = 0;
    let mut j = 1;
    let len = s.len();
    while j < len {
        if s[j] > s[i] {
            i = j;
        } else if s[i] == s[j] {
            let mut flag = false;
            for off in 1..(j - i).min(len - j) {
                if s[i + off] < s[j + off] {
                    i = j;
                    flag = true;
                    break;
                } else if s[i + off] > s[j + off] {
                    j += off;
                    flag = true;
                    break;
                }
            }
            if !flag {
                j += j - i - 1;
            }
        }
        j += 1;
    }
    unsafe { String::from_utf8_unchecked(s[i..].to_vec()) }
}

fn main() {
    assert_eq!(last_substring(String::from("cacacb")), "cb");
    assert_eq!(last_substring(String::from("ababababababab")), "babababababab");
    assert_eq!(last_substring(String::from("abab")), "bab");
    assert_eq!(last_substring(String::from("leetcode")), "tcode");
}
