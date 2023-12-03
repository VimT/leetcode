//! 分类求和并作差

pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    let mut a = 0;
    let mut b = 0;
    for i in 1..=n {
        if i % m == 0 {
            b += i;
        } else {
            a += i;
        }
    }
    a - b
}

pub fn difference_of_sums2(n: i32, m: i32) -> i32 {
    n * (n + 1) / 2 - n / m * (n / m + 1) * m
}

fn main() {
    fn test(func: fn(n: i32, m: i32) -> i32) {
        assert_eq!(func(10, 3), 19);
        assert_eq!(func(5, 6), 15);
        assert_eq!(func(5, 1), -15);
    }
    test(difference_of_sums);
    test(difference_of_sums2);
}
