//! 自除数

use std::sync::Once;

static mut LIST: Option<Box<Vec<i32>>> = None;
static ONCE: Once = Once::new();

pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    unsafe {
        ONCE.call_once(|| {
            let mut nums = vec![];
            for i in 1..=10000 {
                let mut ok = true;
                let mut num = i;
                while num > 0 {
                    if num % 10 == 0 || i % (num % 10) != 0 {
                        ok = false;
                        break;
                    }
                    num /= 10;
                }
                if ok {
                    nums.push(i);
                }
            }
            LIST = Some(Box::new(nums));
        });
        let nums = LIST.as_ref().unwrap();
        let left = nums.binary_search(&left).unwrap_or_else(|x| x);
        let right = nums.binary_search(&right).unwrap_or_else(|x| x - 1);
        nums[left..=right].to_vec()
    }
}

fn main() {
    assert_eq!(self_dividing_numbers(16, 16), vec![]);
    assert_eq!(self_dividing_numbers(1, 22), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]);
    assert_eq!(self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
}
