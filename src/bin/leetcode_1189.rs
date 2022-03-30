//! “气球” 的最大数量

pub fn max_number_of_balloons(text: String) -> i32 {
    let mut cnt = [0; 26];
    let s = text.as_bytes();
    for &ch in s {
        cnt[(ch - b'a') as usize] += 1;
    }
    //balloon
    cnt[(b'b' - b'a') as usize].min(cnt[0]).min(cnt[(b'n' - b'a') as usize]).min(cnt[(b'l' - b'a') as usize] / 2).min(cnt[(b'o' - b'a') as usize] / 2)
}

fn main() {
    assert_eq!(max_number_of_balloons(String::from("nlaebolko")), 1);
    assert_eq!(max_number_of_balloons(String::from("loonbalxballpoon")), 2);
    assert_eq!(max_number_of_balloons(String::from("leetcode")), 0);
}
