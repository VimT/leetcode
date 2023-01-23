//! 隐藏个人信息

pub fn mask_pii(s: String) -> String {
    if let Some(idx) = s.find('@') {
        let s = s.to_ascii_lowercase();
        format!("{}*****{}", &s[..1], &s[idx - 1..])
    } else {
        let s = s.as_bytes();
        let news: Vec<u8> = s.iter().filter(|x| x.is_ascii_digit()).map(|x| *x).collect();
        let last = &news[news.len() - 4..];
        let country_len = news.len() - 10;
        format!("{}***-***-{}", match country_len {
            0 => "".to_string(),
            _ => "+".to_string() + &"*".repeat(country_len) + "-"
        }, String::from_utf8_lossy(last))
    }
}

fn main() {
    assert_eq!(mask_pii(String::from("LeetCode@LeetCode.com")), String::from("l*****e@leetcode.com"));
    assert_eq!(mask_pii(String::from("AB@qq.com")), String::from("a*****b@qq.com"));
    assert_eq!(mask_pii(String::from("1(234)567-890")), String::from("***-***-7890"));
    assert_eq!(mask_pii(String::from("86-(10)12345678")), String::from("+**-***-***-5678"));
}
