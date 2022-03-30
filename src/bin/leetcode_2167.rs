//! 移除所有载有违禁货物车厢所需的最少时间

pub fn minimum_time(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut left = vec![0; len];
    let mut right = vec![0; len];
    let mut cur = 0;
    for i in 0..len {
        if s[i] == b'1' {
            cur = (cur + 2).min(1 + i as i32);
        }
        left[i] = cur;
    }
    cur = 0;
    for i in (0..len).rev() {
        if s[i] == b'1' {
            cur = (cur + 2).min((len - i) as i32);
        }
        right[i] = cur;
    }
    let mut result = left[len - 1];
    for i in 0..len - 1 {
        result = result.min(left[i] + right[i + 1]);
    }
    result
}

pub fn minimum_time_optimise(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = i32::MAX;
    let mut cur = 0;
    for (i, &b) in s.iter().enumerate() {
        if b == b'1' {
            cur = (cur + 2).min(1 + i as i32);
        }
        result = result.min(cur + (len - i - 1) as i32);
    }
    result
}

fn main() {
    assert_eq!(minimum_time_optimise(String::from("1")), 1);
    assert_eq!(minimum_time_optimise(String::from("0")), 0);
    assert_eq!(minimum_time_optimise(String::from("1100101")), 5);
    assert_eq!(minimum_time_optimise(String::from("0010")), 2);
}
