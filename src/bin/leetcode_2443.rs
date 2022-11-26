//! 反转之后的数字和

use std::sync::Once;

static mut RESULT: [bool; 1e5 as usize + 1] = [false; 1e5 as usize + 1];
static ONCE: Once = Once::new();

pub fn sum_of_number_and_reverse(num: i32) -> bool {
    ONCE.call_once(|| unsafe {
        for i in 0..=1e5 as i32 {
            let mut new = 0;
            let mut num = i;
            while num > 0 {
                new = new * 10 + num % 10;
                num /= 10;
            }
            let idx = (new + i) as usize;
            if idx < RESULT.len() {
                RESULT[idx] = true;
            }
        }
    });
    unsafe { RESULT[num as usize] }
}

fn main() {
    fn test(func: fn(num: i32) -> bool) {
        assert_eq!(func(100000), false);
        assert_eq!(func(443), true);
        assert_eq!(func(63), false);
        assert_eq!(func(181), true);
    }
    test(sum_of_number_and_reverse);
}
