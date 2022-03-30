//! 十进制整数的反码

pub fn bitwise_complement(n: i32) -> i32 {
    if n == 0 { return 1; }
    let len = 32 - n.leading_zeros();
    (!n) & ((1 << len) - 1)
}

fn main() {
    assert_eq!(bitwise_complement(1), 0);
    assert_eq!(bitwise_complement(0), 1);
    assert_eq!(bitwise_complement(5), 2);
    assert_eq!(bitwise_complement(7), 0);
    assert_eq!(bitwise_complement(10), 5);
}
