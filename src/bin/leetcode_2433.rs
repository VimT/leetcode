//! 找出前缀异或的原始数组

pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
    let len = pref.len();
    let mut result = vec![0; len];
    result[0] = pref[0];
    for i in 1..len {
        result[i] = pref[i - 1] ^ pref[i];
    }
    result
}

fn main() {
    assert_eq!(find_array(vec![5, 2, 0, 3, 1]), vec![5, 7, 2, 3, 2]);
    assert_eq!(find_array(vec![13]), vec![13]);
}
