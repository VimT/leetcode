//! 6 和 9 组成的最大数字

pub fn maximum69_number(mut num: i32) -> i32 {
    let mut v = vec![];
    while num > 0 {
        v.push(num % 10);
        num /= 10;
    }
    v.reverse();
    for n in &mut v {
        if *n == 6 {
            *n = 9;
            break;
        }
    }
    num = 0;
    for w in v {
        num = num * 10 + w;
    }
    num
}

fn main() {
    fn test(func: fn(num: i32) -> i32) {
        assert_eq!(func(9669), 9969);
        assert_eq!(func(9996), 9999);
        assert_eq!(func(9999), 9999);
    }
    test(maximum69_number);
}
