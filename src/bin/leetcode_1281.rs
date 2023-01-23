//! 整数的各位积和之差

pub fn subtract_product_and_sum(mut n: i32) -> i32 {
    let mut ji = 1;
    let mut sum = 0;
    while n > 0 {
        let num = n % 10;
        ji *= num;
        sum += num;
        n /= 10;
    }
    ji - sum
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(234), 15);
        assert_eq!(func(4421), 21);
    }
    test(subtract_product_and_sum);
}
