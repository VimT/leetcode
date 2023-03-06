//! 递枕头

pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let mut reverse = false;
    let mut cur = 1;
    for _ in 0..time {
        if reverse {
            cur -= 1;
            if cur == 1 {
                reverse = !reverse;
            }
        } else {
            cur += 1;
            if cur == n {
                reverse = !reverse;
            }
        }
    }
    cur
}

fn main() {
    fn test(func: fn(n: i32, time: i32) -> i32) {
        assert_eq!(func(4, 5), 2);
        assert_eq!(func(3, 2), 3);
    }
    test(pass_the_pillow);
}
