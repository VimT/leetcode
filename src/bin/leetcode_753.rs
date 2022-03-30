//! 破解保险箱

use std::collections::BTreeSet;

/// 欧拉图  Hierholzer 算法
pub fn crack_safe(n: i32, k: i32) -> String {
    fn dfs(mut cur: Vec<u8>, k: u8, set: &mut BTreeSet<Vec<u8>>, result: &mut Vec<u8>) {
        for i in 0..k {
            cur.push(i + b'0');
            if !set.contains(&cur) {
                set.insert(cur.clone());
                dfs(cur[1..].to_vec(), k, set, result);
                result.push(i + b'0');
            }
            cur.pop();
        }
    }
    let mut set = BTreeSet::new();
    let start = vec![b'0'; n as usize - 1];
    let mut result = Vec::with_capacity(2 * (k as usize).pow(n as u32));
    dfs(start.clone(), k as u8, &mut set, &mut result);
    result.extend(start);
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(crack_safe(1, 2), "10");
    assert_eq!(crack_safe(2, 2), "01100");
    assert_eq!(crack_safe(3, 2), "0011101000");
}