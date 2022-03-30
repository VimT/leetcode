//! 解码异或后的排列

pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
    let len = encoded.len();
    let n = len as i32 + 1;
    let mut total = 0;
    for i in 1..=n {
        total ^= i;
    }
    let mut odd = 0;
    for i in (1..len).step_by(2) {
        odd ^= encoded[i];
    }
    let mut result = vec![0; len + 1];
    result[0] = total ^ odd;
    for i in 0..len as usize {
        result[i + 1] = result[i] ^ encoded[i];
    }
    result
}

fn main() {
    assert_eq!(decode(vec![2, 1]), vec![1, 3, 2]);
    assert_eq!(decode(vec![3, 1]), vec![1, 2, 3]);
    assert_eq!(decode(vec![6, 5, 4, 6]), vec![2, 4, 1, 5, 3]);
}
