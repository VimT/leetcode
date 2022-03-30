//! 消除游戏

pub fn last_remaining(mut n: i32) -> i32 {
    let mut start = 1;
    let mut step = 1;
    let mut l2r = true;
    while n > 1 {
        if l2r || (n & 1) == 1 {
            start += step;
        }
        n >>= 1;
        step <<= 1;
        l2r = !l2r;
    }
    start
}

fn main() {
    assert_eq!(last_remaining(9), 6);
}

