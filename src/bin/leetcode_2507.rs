//! 使用质因数之和替换后可以取到的最小值


use std::sync::Once;

static mut PRIME: Vec<i32> = vec![];
static ONCE: Once = Once::new();

pub fn smallest_value(n: i32) -> i32 {
    unsafe {
        ONCE.call_once(|| {
            let mut result = vec![2, 3, 5, 7, 11, 13];
            for i in 17..=100000 {
                let mut ok = true;
                for &num in &result {
                    if num * num > i {
                        break;
                    }
                    if i % num == 0 {
                        ok = false;
                        break;
                    }
                }
                if ok { result.push(i); }
            }

            PRIME = result;
        });
        let mut result = 0;
        let mut p = n;
        while p > 1 {
            if PRIME.binary_search(&p).is_ok() {
                result += p;
                break;
            }
            for &num in &PRIME {
                if num * num > p { break; }
                if p % num == 0 {
                    result += num;
                    p /= num;
                    break;
                }
            }
        }
        if result < n { smallest_value(result) } else { n }
    }
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(15), 5);
        assert_eq!(func(3), 3);
        assert_eq!(func(4), 4);
    }
    test(smallest_value);
}
