//! 易混淆数 II

use std::sync::Once;

static mut NUM: Option<Vec<i32>> = None;
static ONCE: Once = Once::new();
static NUMS: [(i32, i32); 5] = [(0, 0), (1, 1), (6, 9), (8, 8), (9, 6)];

fn dfs(cur: i32, cur_len: usize, rev: i32, result: &mut Vec<i32>, target_len: usize) {
    if cur_len == target_len {
        if cur != rev {
            result.push(cur);
        }
        return;
    }
    for &(num, rev_num) in &NUMS[if cur_len == 0 { 1 } else { 0 }..] {
        dfs(cur * 10 + num, cur_len + 1, rev_num * 10_i32.pow(cur_len as u32) + rev, result, target_len);
    }
}

pub fn confusing_number_ii(n: i32) -> i32 {
    unsafe {
        ONCE.call_once(|| {
            let mut num = Vec::with_capacity(1950628);
            for i in 1..=9 {
                dfs(0, 0, 0, &mut num, i);
            }
            num.push(1000000000);
            NUM = Some(num);
        });
        let num = NUM.as_ref().unwrap();
        num.binary_search(&(n + 1)).unwrap_or_else(|x| x) as i32
    }
}


fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(100000000), 389627);
        assert_eq!(func(999959), 15411);
        assert_eq!(func(20), 6);
        assert_eq!(func(100), 19);
    }
    test(confusing_number_ii);
}
