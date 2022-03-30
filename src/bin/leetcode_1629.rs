//! 按键持续时间最长的键

pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
    let s = keys_pressed.as_bytes();
    let len = s.len();
    let mut last = 0;
    let mut max_time = 0;
    let mut result = 0;
    for i in 0..len {
        let time = release_times[i] - last;
        if time > max_time || (time == max_time && s[i] > result) {
            max_time = time;
            result = s[i];
        }
        last = release_times[i];
    }
    result as char
}

fn main() {
    assert_eq!(slowest_key(vec![9, 29, 49, 50], String::from("cbcd")), 'c');
    assert_eq!(slowest_key(vec![12, 23, 36, 46, 62], String::from("spuda")), 'a');
}
