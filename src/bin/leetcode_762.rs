//! 二进制表示中质数个计算置位

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut result = 0;
    for i in left..=right {
        if matches!(i.count_ones(), 2|3|5|7|11|13|17|19|23|29|31) { result += 1 }
    }
    result
}

fn main() {
    assert_eq!(count_prime_set_bits(6, 10), 4);
    assert_eq!(count_prime_set_bits(10, 15), 5);
}
