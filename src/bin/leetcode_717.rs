//! 1比特与2比特字符

pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let len = bits.len();
    let mut i = 0;
    while i < len - 1 {
        if bits[i] == 1 {
            i += 2;
        } else {
            i += 1;
        }
    }
    i == len - 1 && bits[i] == 0
}

fn main() {
    assert_eq!(is_one_bit_character(vec![1, 0, 0]), true);
    assert_eq!(is_one_bit_character(vec![1, 1, 1, 0]), false);
}
