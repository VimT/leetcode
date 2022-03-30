//! 增长的内存泄露

pub fn mem_leak(mut memory1: i32, mut memory2: i32) -> Vec<i32> {
    let mut cur = 1;
    while memory1 >= cur || memory2 >= cur {
        *(&mut memory2).max(&mut memory1) -= cur;
        cur += 1;
    }
    vec![cur, memory1, memory2]
}

fn main() {
    assert_eq!(mem_leak(8, 11), vec![6, 0, 4]);
    assert_eq!(mem_leak(2, 2), vec![3, 1, 0]);
}
