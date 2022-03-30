//! 汉明距离


pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut result = 0;
    for i in 0..32 {
        if x >> i & 1 != y >> i & 1 {
            result += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(hamming_distance(1, 4), 2);
    assert_eq!(hamming_distance(3, 1), 1);
}
