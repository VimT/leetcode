//! 位1的个数

#[allow(non_snake_case)]
pub fn hammingWeight(n: u32) -> i32 {
    n.count_ones() as i32
}

fn main() {
    fn test(func: fn(n: u32) -> i32) {
        assert_eq!(func(0b00000000000000000000000000001011), 3);
        assert_eq!(func(0b00000000000000000000000010000000), 1);
        assert_eq!(func(0b11111111111111111111111111111101), 31);
    }
    test(hammingWeight);
}
