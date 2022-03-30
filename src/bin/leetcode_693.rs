//! 交替位二进制数

pub fn has_alternating_bits(mut n: i32) -> bool {
    let mut last = n & 1;
    n >>= 1;
    while n > 0 {
        let this = n & 1;
        if this == last {
            return false;
        }
        n >>= 1;
        last = this;
    }
    true
}

fn main() {
    assert_eq!(has_alternating_bits(5), true);
    assert_eq!(has_alternating_bits(7), false);
    assert_eq!(has_alternating_bits(11), false);
}
