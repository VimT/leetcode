//! 奇偶位数

pub fn even_odd_bit(n: i32) -> Vec<i32> {
    let mut odd = 0;
    let mut even = 0;
    for i in 0..32 {
        if n >> i & 1 == 1 {
            if i & 1 == 1 {
                odd += 1;
            } else {
                even += 1;
            }
        }
    }
    vec![even, odd]
}

fn main() {
    fn test(func: fn(n: i32) -> Vec<i32>) {
        assert_eq!(func(17), vec![2, 0]);
        assert_eq!(func(2), vec![0, 1]);
    }
    test(even_odd_bit);
}
