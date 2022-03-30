//! k 镜像数字的和


use std::collections::HashSet;
use std::sync::Once;

static mut TEN: Option<Box<HashSet<i64>>> = None;
static ONCE: Once = Once::new();

pub fn k_mirror(k: i32, n: i32) -> i64 {
    fn inner(cur: &mut Vec<u8>, idx: usize, result: &mut Vec<i64>, k: u8) {
        let len = cur.len();
        if idx > (len - 1) / 2 {
            let mut num = 0_i64;
            for i in 0..len {
                num += cur[len - i - 1] as i64 * (k as i64).pow((len - i - 1) as u32);
            }
            result.push(num);
            return;
        }
        let start: u8 = if idx == 0 { 1 } else { 0 };
        for i in start..k {
            cur[idx] = i;
            cur[len - idx - 1] = i;
            inner(cur, idx + 1, result, k);
        }
        cur[idx] = start;
        cur[len - idx - 1] = start;
    }

    unsafe {
        ONCE.call_once(|| {
            let mut cur_len = 1;
            let n = 644545;
            let mut nums = Vec::with_capacity(2 * n);
            while nums.len() < n {
                let mut cur = vec![0; cur_len];
                inner(&mut cur, 0, &mut nums, 10);
                cur_len += 1;
            }
            TEN = Some(Box::new(nums.into_iter().collect()));
        });
        let mut cur_len = 1;
        let n = n as usize;
        let mut nums = Vec::with_capacity(2 * n);
        while nums.len() < n {
            let mut cur = vec![0; cur_len];
            let mut len_nums = vec![];
            inner(&mut cur, 0, &mut len_nums, k as u8);
            cur_len += 1;
            for i in len_nums {
                if TEN.as_ref().unwrap().contains(&i) {
                    nums.push(i);
                }
            }
        }
        nums[..n].iter().sum()
    }
}

fn main() {
    assert_eq!(k_mirror(2, 5), 25);
    assert_eq!(k_mirror(3, 7), 499);
    assert_eq!(k_mirror(7, 17), 20379000);
}
