//! 数组大小减半

use std::collections::HashMap;

pub fn min_set_size(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for num in arr {
        *cnt.entry(num).or_default() += 1;
    }
    let mut cnt: Vec<i32> = cnt.into_values().collect();
    cnt.sort_unstable();
    let mut cur = 0;
    let mut result = 0;
    let tlen = len >> 1;
    for num in cnt.into_iter().rev() {
        cur += num as usize;
        result += 1;
        if cur >= tlen {
            break;
        }
    }
    result
}

/// 酷炫的流式写法
pub fn min_set_size2(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let count_by_num = {
        let mut m = [0usize; 100001];
        arr.into_iter().for_each(|v| m[v as usize] += 1);
        m
    };
    let freq = {
        let mut freq = count_by_num
            .iter()
            .filter(|&&v| v > 0)
            .collect::<Vec<_>>();
        freq.sort_unstable();
        freq
    };
    let min_elements = freq
        .into_iter()
        .rev()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .take_while(|x| 2 * x < n)
        .count();
    (min_elements + 1) as i32
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]), 2);
        assert_eq!(func(vec![7, 7, 7, 7, 7, 7]), 1);
    }
    test(min_set_size);
    test(min_set_size2);
}
