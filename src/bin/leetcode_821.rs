//! 字符的最短距离

pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let mut left = 1e5 as i32;
    let mut right = 0;
    let s = s.as_bytes();
    let c = c as u8;
    let len = s.len();
    let mut result = vec![0; len];
    for i in 0..len {
        if s[i] == c {
            right = i as i32;
            break;
        }
    }
    for i in 0..len {
        result[i] = (i as i32 - left).abs().min((right - i as i32).abs());
        if i as i32 == right {
            left = right;
            for j in i + 1..len {
                if s[j] == c {
                    right = j as i32;
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    assert_eq!(shortest_to_char(String::from("loveleetcode"), 'e'), vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]);
    assert_eq!(shortest_to_char(String::from("aaab"), 'b'), vec![3, 2, 1, 0]);
}
