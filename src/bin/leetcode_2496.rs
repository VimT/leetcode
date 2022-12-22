//! 数组中字符串的最大值

use leetcode::svec;

pub fn maximum_value(strs: Vec<String>) -> i32 {
    strs.into_iter().map(|x| {
        if let Ok(x) = x.parse::<i32>() {
            x
        } else {
            x.len() as i32
        }
    }).max().unwrap()
}

fn main() {
    assert_eq!(maximum_value(svec!["alic3","bob","3","4","00000"]), 5);
    assert_eq!(maximum_value(svec!["1","01","001","0001"]), 1);
}
