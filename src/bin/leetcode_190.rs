//! 颠倒二进制位

pub fn reverse_bits(mut x: u32) -> u32 {
    let mut result = 0;
    for i in (0..32).rev() {
        result |= (x & 1) << i;
        x >>= 1;
    }
    result
}

fn main() {
    fn test(func: fn(x: u32) -> u32) {
        assert_eq!(func(0b00000010100101000001111010011100), 0b00111001011110000010100101000000);
        assert_eq!(func(0b11111111111111111111111111111101), 0b10111111111111111111111111111111);
    }
    test(reverse_bits);
}
