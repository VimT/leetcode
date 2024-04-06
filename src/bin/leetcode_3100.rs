//! 换水问题 II

pub fn max_bottles_drunk(num_bottles: i32, mut num_exchange: i32) -> i32 {
    let mut result = num_bottles;
    let mut empty = num_bottles;
    while empty >= num_exchange {
        empty -= num_exchange;
        empty += 1;
        result += 1;
        num_exchange += 1;
    }
    result
}

fn main() {
    fn test(func: fn(num_bottles: i32, num_exchange: i32) -> i32) {
        assert_eq!(func(13, 6), 15);
        assert_eq!(func(10, 3), 13);
    }
    test(max_bottles_drunk);
}
