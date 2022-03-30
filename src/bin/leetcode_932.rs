//! 漂亮数组

use std::collections::HashMap;

pub fn beautiful_array(n: i32) -> Vec<i32> {
    fn inner(n: usize, cache: &mut HashMap<usize, Vec<i32>>) -> Vec<i32> {
        if let Some(r) = cache.get(&n) {
            return r.clone();
        }
        if n == 1 { return vec![1]; }
        let mut result = Vec::with_capacity(n);
        result.extend(inner((n + 1) / 2, cache).iter().map(|x| 2 * *x - 1));
        result.extend(inner(n / 2, cache).iter().map(|x| 2 * *x));
        cache.insert(n, result.clone());
        result
    }
    inner(n as usize, &mut HashMap::new())
}

fn main() {
    assert_eq!(beautiful_array(4), vec![1, 3, 2, 4]);
    assert_eq!(beautiful_array(5), vec![1, 5, 3, 2, 4]);
}
