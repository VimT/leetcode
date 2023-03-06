//! 将整数按权重排序

use std::sync::Once;

static mut DATA: [i32; 1001] = [0; 1001];
static ONCE: Once = Once::new();

pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    unsafe {
        ONCE.call_once(|| {
            for i in 1..10 {
                DATA[1 << i] = i;
            }
            let mut past = vec![];
            for mut i in 2..=1000 {
                while i > 1 {
                    if i < 1001 && DATA[i] != 0 { break; }
                    past.push(i);
                    if i & 1 == 1 {
                        i = i * 3 + 1;
                    } else {
                        i >>= 1;
                    }
                }
                let mut a = DATA[i] + 1;
                while !past.is_empty() {
                    let last = past.pop().unwrap();
                    if last < 1001 { DATA[last] = a; }
                    a += 1;
                }
            }
        });
        let mut arr: Vec<i32> = (lo..=hi).collect();
        arr.sort_unstable_by_key(|&x| (DATA[x as usize], x));
        arr[(k - 1) as usize]
    }
}

fn main() {
    fn test(func: fn(lo: i32, hi: i32, k: i32) -> i32) {
        assert_eq!(func(12, 15, 2), 13);
        assert_eq!(func(7, 11, 4), 7);
    }
    test(get_kth);
}
