//! 回文素数

use std::sync::Once;

static mut M: Option<Box<Vec<i32>>> = None;
static ONCE: Once = Once::new();

pub fn prime_palindrome(n: i32) -> i32 {
    unsafe {
        fn inner(cur: &mut Vec<u8>, idx: usize, result: &mut Vec<i32>, stop: i32) -> bool {
            let len = cur.len();
            if idx > (len - 1) / 2 {
                let mut num = 0;
                for i in 0..len {
                    num = num * 10 + cur[i] as i32;
                }
                if num == 1 { return true; }
                for i in 2..=(num as f64).sqrt() as i32 {
                    if num % i == 0 {
                        return true;
                    }
                }
                result.push(num);
                if num > stop {
                    return false;
                }
                return true;
            }
            let start: u8 = if idx == 0 { 1 } else { 0 };
            for i in start..10 {
                cur[idx] = i;
                cur[len - idx - 1] = i;
                if !inner(cur, idx + 1, result, stop) { return false; };
            }
            cur[idx] = start;
            cur[len - idx - 1] = start;
            true
        }
        ONCE.call_once(|| {
            let mut nums = Vec::with_capacity(2e5 as usize);
            let mut cur_len = 1;
            while *nums.last().unwrap_or(&0) < 2e8 as i32 {
                let mut cur = vec![0; cur_len];
                inner(&mut cur, 0, &mut nums, 2e8 as i32);
                cur_len += 1;
                if cur_len == 8 { cur_len += 1; }
            }
            // println!("{:?}", nums);
            M = Some(Box::new(nums));
        });
        let m = M.as_ref().unwrap();
        m[m.binary_search(&n).unwrap_or_else(|x| x)]
    }
}

fn main() {
    assert_eq!(prime_palindrome(1), 2);
    assert_eq!(prime_palindrome(6), 7);
    assert_eq!(prime_palindrome(8), 11);
    assert_eq!(prime_palindrome(13), 101);
}
