//! 找出第 K 个字符 II

pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
    let mut result = b'a';
    let k = k - 1;
    for i in 0..64 {
        if k >> i & 1 == 1 {
            if operations[i] == 1 {
                result += 1;
                if result == b'z' + 1 {
                    result = b'a';
                }
            }
        }
    }
    result as char
}

fn main() {
    fn test(func: fn(k: i64, operations: Vec<i32>) -> char) {
        assert_eq!(func(5, vec![0, 0, 0]), 'a');
        assert_eq!(func(10, vec![0, 1, 0, 1]), 'b');
    }
    test(kth_character);
}
