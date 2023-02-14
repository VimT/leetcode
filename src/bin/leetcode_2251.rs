//! 花期内花的数目

use std::cmp::Ordering;
use std::collections::BTreeMap;

/// 差分
pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, persons: Vec<i32>) -> Vec<i32> {
    let mut diff = BTreeMap::new();
    for flower in flowers {
        *diff.entry(flower[0]).or_insert(0) += 1;
        *diff.entry(flower[1] + 1).or_insert(0) -= 1;
    }
    let times: Vec<i32> = diff.keys().cloned().collect();
    let mut i = 0;
    let mut cursum = 0;
    let mut persons: Vec<(usize, i32)> = persons.into_iter().enumerate().collect();
    persons.sort_unstable_by_key(|x| x.1);
    let mut result = vec![0; persons.len()];
    for (idx, p) in persons {
        while i < times.len() && times[i] <= p {
            cursum += diff[&times[i]];
            i += 1;
        }
        result[idx] = cursum;
    }
    result
}

/// 二分，在时间点前 开花的减去凋落的
pub fn full_bloom_flowers2(flowers: Vec<Vec<i32>>, persons: Vec<i32>) -> Vec<i32> {
    let mut starts: Vec<i32> = flowers.iter().map(|x| x[0]).collect();
    let mut ends: Vec<i32> = flowers.iter().map(|x| x[1]).collect();
    starts.sort_unstable();
    ends.sort_unstable();
    persons.into_iter().map(|x| {
        let r = starts.binary_search_by(|mid| mid.cmp(&x).then(Ordering::Less)).unwrap_err();
        let l = ends.binary_search_by(|mid| mid.cmp(&x).then(Ordering::Greater)).unwrap_err();
        (r - l) as i32
    }).collect()
}

fn main() {
    fn test(func: fn(flowers: Vec<Vec<i32>>, persons: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]], vec![2, 3, 7, 11]), vec![1, 2, 2, 2]);
        assert_eq!(func(vec![vec![1, 10], vec![3, 3]], vec![3, 3, 2]), vec![2, 2, 1]);
    }
    test(full_bloom_flowers);
    test(full_bloom_flowers2);
}
