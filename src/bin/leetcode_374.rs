//! 猜数字大小


static mut TARGET: i32 = 0;

unsafe fn guess(num: i32) -> i32 {
    return if num < TARGET { 1 } else if num > TARGET { -1 } else { 0 };
}

#[allow(non_snake_case)]
unsafe fn guessNumber(n: i32) -> i32 {
    let mut left = 1;
    let mut right = n;
    while left < right {
        let mid = ((right - left) >> 1) + left;
        if guess(mid) == 1 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn main() {
    let help = |x: i32| unsafe {
        TARGET = x;
        assert_eq!(guessNumber(x), x);
    };
    help(i32::MAX);
    help(1);
}
