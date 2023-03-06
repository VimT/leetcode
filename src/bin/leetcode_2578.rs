//! 最小和分割

pub fn split_num(mut num: i32) -> i32 {
    let mut arr = vec![];
    while num > 0 {
        arr.push(num % 10);
        num /= 10;
    }
    arr.sort_unstable();
    let mut a = 0;
    let mut b = 0;
    for i in 0..arr.len() {
        if i & 1 == 0 {
            a = a * 10 + arr[i];
        } else {
            b = b * 10 + arr[i];
        }
    }
    a + b
}

fn main() {
    fn test(func: fn(num: i32) -> i32) {
        assert_eq!(func(4325), 59);
        assert_eq!(func(687), 75);
    }
    test(split_num);
}
