//! 所有子字符串美丽值之和

pub fn beauty_sum(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    if len < 3 { return 0; }
    let mut result = 0;
    for i in 0..len - 2 {
        let mut m = vec![0; 26];
        for j in i..i + 2 {
            m[(s[j] - b'a') as usize] += 1;
        }
        for j in i + 2..len {
            m[(s[j] - b'a') as usize] += 1;
            let mut min = len as i32;
            let mut max = 0;
            for &k in &m {
                if k > 0 {
                    min = min.min(k);
                    max = max.max(k);
                }
            }
            if max > min {
                result += max - min;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(beauty_sum(String::from("a")), 0);
    assert_eq!(beauty_sum(String::from("aabcb")), 5);
    assert_eq!(beauty_sum(String::from("aabcbaa")), 17);
}