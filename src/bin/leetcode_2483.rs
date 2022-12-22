//! 商店的最少代价

pub fn best_closing_time(customers: String) -> i32 {
    let s = customers.as_bytes();
    let len = s.len();
    let mut pre_n = vec![0; len + 1];
    let mut suf_y = vec![0; len + 1];
    for i in 0..len {
        pre_n[i + 1] = pre_n[i] + (s[i] == b'N') as i32;
    }
    for i in (0..len).rev() {
        suf_y[i] = suf_y[i + 1] + (s[i] == b'Y') as i32;
    }
    let mut result = 0;
    let mut min = i32::MAX;
    for i in 0..=len {
        if pre_n[i] + suf_y[i] < min {
            result = i as i32;
            min = pre_n[i] + suf_y[i];
        }
    }
    result
}

fn main() {
    assert_eq!(best_closing_time(String::from("YYNY")), 2);
    assert_eq!(best_closing_time(String::from("NNNN")), 0);
    assert_eq!(best_closing_time(String::from("YYYY")), 4);
}