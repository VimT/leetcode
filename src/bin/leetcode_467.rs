//! 环绕字符串中唯一的子字符串

pub fn find_substring_in_wrapround_string(p: String) -> i32 {
    let p = p.as_bytes();
    let len = p.len();
    let mut ll = 1;
    let mut start_max = [0; 26];
    for i in 0..len {
        ll -= 1;
        while i + ll < len && p[i + ll] == ((((p[i] - b'a') as usize + ll) % 26) as u8 + b'a') {
            ll += 1;
        }
        start_max[(p[i] - b'a') as usize] = start_max[(p[i] - b'a') as usize].max(ll);
    }
    start_max.iter().sum::<usize>() as i32
}

fn main() {
    assert_eq!(find_substring_in_wrapround_string(String::from("a")), 1);
    assert_eq!(find_substring_in_wrapround_string(String::from("cac")), 2);
    assert_eq!(find_substring_in_wrapround_string(String::from("zab")), 6);
}
