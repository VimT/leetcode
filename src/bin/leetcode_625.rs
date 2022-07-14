//! 最小因式分解

pub fn smallest_factorization(mut num: i32) -> i32 {
    if num == 1 { return 1; }
    let mut result = vec![];
    for i in (2..10).rev() {
        while num % i == 0 {
            num /= i;
            result.push(i);
        }
    }
    if num > 1 {
        return 0;
    }
    result.reverse();
    let mut num = 0i32;
    for i in result {
        match num.checked_mul(10).and_then(|num| num.checked_add(i)) {
            None => { return 0; }
            Some(n) => { num = n }
        }
    }
    num
}

fn main() {
    fn test(func: fn(num: i32) -> i32) {
        assert_eq!(func(18000000), 0);
        assert_eq!(func(48), 68);
        assert_eq!(func(15), 35);
    }
    test(smallest_factorization);
}
