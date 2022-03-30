//! 二进制手表

use std::sync::Once;

use leetcode::svec;

static mut MAP: Option<Vec<Vec<String>>> = None;
static ONCE: Once = Once::new();

pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    unsafe {
        ONCE.call_once(|| {
            let mut m = vec![vec![]; 11];
            for i in 0i32..1 << 11 {
                let hour = i >> 6;
                let minutes = i & (0b111111);
                if hour < 12 && minutes < 60 {
                    let idx = i.count_ones();
                    m[idx as usize].push(format!("{}:{:02}", hour, minutes));
                }
            }
            MAP = Some(m);
        });
        MAP.as_ref().unwrap()[turned_on as usize].clone()
    }
}

fn main() {
    assert_eq!(read_binary_watch(1), svec!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]);
    let v: Vec<String> = vec![];
    assert_eq!(read_binary_watch(9), v);
    assert_eq!(read_binary_watch(10), v);
}
