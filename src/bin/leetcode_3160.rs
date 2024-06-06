//! 所有球里面不同颜色的数目

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn query_results(_: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut num_color: HashMap<i32, i32> = HashMap::new(); // 第k个球的颜色为color
    let mut color_cnt: HashMap<i32, i32> = HashMap::new(); // color k 有多少个球
    queries.into_iter().map(|q| {
        let (num, color) = (q[0], q[1]);
        if let Some(&before_color) = num_color.get(&num) {
            if let Entry::Occupied(mut v) = color_cnt.entry(before_color) {
                *v.get_mut() -= 1;
                if *v.get() == 0 {
                    v.remove();
                }
            }
        }
        num_color.insert(num, color);
        *color_cnt.entry(color).or_default() += 1;
        color_cnt.len() as i32
    }).collect()
}

fn main() {
    fn test(func: fn(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]), vec![1, 2, 2, 3]);
        assert_eq!(func(4, vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]]), vec![1, 2, 2, 3, 4]);
    }
    test(query_results);
}
