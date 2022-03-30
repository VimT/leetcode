//! 有序队列

pub fn orderly_queue(s: String, k: i32) -> String {
    let mut s = s.into_bytes();
    if k == 1 {
        let len = s.len();
        let oris = s.clone();
        for i in 0..len {
            let mut news = oris[i..].to_vec();
            news.extend_from_slice(&oris[..i]);
            if news < s {
                s = news;
            }
        }
    } else {
        s.sort_unstable();
    }
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(orderly_queue(String::from("nhtq"), 1), String::from("htqn"));
    assert_eq!(orderly_queue(String::from("cba"), 1), String::from("acb"));
    assert_eq!(orderly_queue(String::from("baaca"), 3), String::from("aaabc"));
}
