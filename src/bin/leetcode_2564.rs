//! 子字符串异或查询

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let s = s.as_bytes();
    let len = s.len();
    let mut map: HashMap<i32, (i32, i32)> = HashMap::with_capacity(len * 31);
    for i in 0..len {
        let mut num = 0;
        for j in i..len.min(i + 32) {
            num = num * 2 + (s[j] - b'0') as i32;
            if let Entry::Vacant(v) = map.entry(num) {
                v.insert((i as i32, j as i32));
            }
            if num == 0 { break; }
        }
    }
    queries.into_iter().map(|q| {
        map.get(&(q[0] ^ q[1])).map(|x| vec![x.0, x.1]).unwrap_or(vec![-1, -1])
    }).collect()
}

fn main() {
    fn test(func: fn(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(String::from("101101"), vec![vec![0, 5], vec![1, 2]]), vec![vec![0, 2], vec![2, 3]]);
        assert_eq!(func(String::from("0101"), vec![vec![12, 8]]), vec![vec![-1, -1]]);
        assert_eq!(func(String::from("1"), vec![vec![4, 5]]), vec![vec![0, 0]]);
    }
    test(substring_xor_queries);
}
