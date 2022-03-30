//! 二倍数对数组

use std::collections::BTreeMap;

pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
    let mut map = BTreeMap::new();
    for num in arr {
        *map.entry(num).or_insert(0i32) += 1;
    }
    while let Some((&k, &v)) = map.range(..).next() {
        map.remove(&k);
        if k == 0 {
            if v & 1 == 1 { return false; }
            continue;
        }
        let next_key = if k < 0 {
            if k & 1 == 1 {
                return false;
            }
            k / 2
        } else {
            k * 2
        };
        if let Some(v2) = map.get_mut(&next_key) {
            *v2 -= v;
            if *v2 < 0 { return false; }
            if *v2 == 0 { map.remove(&next_key); }
        } else {
            return false;
        }
    }
    true
}

fn main() {
    assert_eq!(can_reorder_doubled(vec![4, -2, 2, -4]), true);
    assert_eq!(can_reorder_doubled(vec![2, 4, 0, 0, 8, 1]), true);
    assert_eq!(can_reorder_doubled(vec![3, 1, 3, 6]), false);
    assert_eq!(can_reorder_doubled(vec![2, 1, 2, 6]), false);
}
