//! 计算字符串的数字和

pub fn digit_sum(s: String, k: i32) -> String {
    let mut s = s.into_bytes();
    let k = k as usize;
    while s.len() > k {
        let mut news = vec![];
        for c in s.chunks(k) {
            let mut this_sum = 0;
            for &ch in c {
                this_sum += (ch - b'0') as i32;
            }
            news.extend_from_slice(this_sum.to_string().as_bytes());
        }
        s = news;
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(digit_sum(String::from("11111222223"), 3), String::from("135"));
    assert_eq!(digit_sum(String::from("00000000"), 3), String::from("000"));
}
