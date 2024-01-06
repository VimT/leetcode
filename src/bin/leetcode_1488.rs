//! 避免洪水泛滥

use std::collections::{BTreeSet, HashMap};

/// 贪心的在湖填满的第一个空闲日抽干
pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let len = rains.len();
    let mut free = BTreeSet::new();
    let mut lake: HashMap<i32, usize> = HashMap::new();
    let mut result = vec![-1; len];
    for i in 0..len {
        if rains[i] > 0 {
            if let Some(&last) = lake.get(&rains[i]) {
                if let Some(&s) = free.range(last..i).next() {
                    result[s] = rains[i];
                    free.remove(&s);
                } else {
                    return vec![];
                }
            }
            lake.insert(rains[i], i);
        } else {
            free.insert(i);
        }
    }
    for x in free {
        result[x] = 1;
    }
    result
}

fn main() {
    fn test(func: fn(rains: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![1,0,2,3,0,1,2]), vec![-1,1,-1,-1,2,-1,-1]);
        assert_eq!(func(vec![1, 2, 3, 4]), vec![-1, -1, -1, -1]);
        assert_eq!(func(vec![1, 2, 0, 0, 2, 1]), vec![-1, -1, 2, 1, -1, -1]);
        assert_eq!(func(vec![1, 2, 0, 1, 2]), vec![]);
    }
    test(avoid_flood);
}
