//! 阿姆斯特朗数

pub fn is_armstrong(n: i32) -> bool {
    let mut num = n;
    let mut arr = vec![];
    while num > 0 {
        arr.push(num % 10);
        num /= 10;
    }
    let len = arr.len() as u32;
    let mut sum = 0;
    for num in arr {
        sum += num.pow(len);
    }
    sum == n
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(153), true);
        assert_eq!(func(123), false);
    }
    test(is_armstrong);
}
