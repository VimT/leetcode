//! 统计距离为 k 的点对

use std::collections::HashMap;

pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut yy: HashMap<(i32, i32), i32> = HashMap::new();
    for coo in &coordinates {
        *yy.entry((coo[0], coo[1])).or_default() += 1;
    }
    let mut result = 0;
    for coo in coordinates {
        let (x, y) = (coo[0], coo[1]);
        *yy.get_mut(&(x, y)).unwrap() -= 1;
        for a in 0..=k {
            result += yy.get(&(x ^ a, y ^ (k - a))).copied().unwrap_or(0);
        }
    }
    result
}

fn main() {
    fn test(func: fn(coordinates: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![1, 2], vec![4, 2], vec![1, 3], vec![5, 2]], 5), 2);
        assert_eq!(func(vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3]], 0), 10);
    }
    test(count_pairs);
}
