//! 检查相同字母间的距离

pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
    let mut last_seen = [None; 26];
    for (i, &ch) in s.as_bytes().iter().enumerate() {
        if let Some(last) = last_seen[(ch - b'a') as usize] {
            if distance[(ch - b'a') as usize] as usize != i - last - 1 {
                return false;
            }
        }
        last_seen[(ch - b'a') as usize] = Some(i);
    }
    true
}

fn main() {
    assert_eq!(check_distances(String::from("abaccb"), vec![1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), true);
    assert_eq!(check_distances(String::from("aa"), vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), false);
}
