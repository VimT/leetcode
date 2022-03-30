//! 用 Rand7() 实现 Rand10()
use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

static mut CUR: i64 = 9999;

pub fn rand7() -> i32 {
    unsafe {
        CUR = (314159269 * CUR + 453806245) % 2147483648;
        return (CUR as f64 / 2147483648_f64 * 7.0 + 1.0) as i32;
    }
}

/// (randX() - 1)*Y + randY() 可以等概率的生成[1, X * Y]范围的随机数
pub fn rand10() -> i32 {
    let mut num = (rand7() - 1) * 7 + rand7();
    while num > 40 {
        num = (rand7() - 1) * 7 + rand7();
    }
    return 1 + num % 10;
}

fn main() {
    let mut count = vec![0; 11];

    for _ in 0..10000 {
        count[rand10() as usize] += 1;
    }
    for (i, v) in count.into_iter().enumerate() {
        println!("{}: {}", i, v);
    }
}