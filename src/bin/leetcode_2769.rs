//! 找出最大的可达成数字

pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
    num + t * 2
}

fn main() {
    fn test(func: fn(num: i32, t: i32) -> i32) {
        assert_eq!(func(4, 1), 6);
        assert_eq!(func(3, 2), 7);
    }
    test(the_maximum_achievable_x);
}
