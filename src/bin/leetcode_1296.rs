//! 划分数组为连续数字的集合

use std::collections::BTreeMap;

pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    let k = k as usize;
    if len % k != 0 { return false; }
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    for num in nums {
        *map.entry(num).or_default() += 1;
    }
    for _ in 0..len / k {
        let start = *map.range(..).next().unwrap().0;
        for cur in start..start + k as i32 {
            if map.contains_key(&cur) {
                *map.get_mut(&cur).unwrap() -= 1;
                if map[&cur] == 0 {
                    map.remove(&cur);
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> bool) {
        assert_eq!(func(vec![1, 2, 3, 3, 4, 4, 5, 6], 4), true);
        assert_eq!(func(vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3), true);
        assert_eq!(func(vec![1, 2, 3, 4], 3), false);
    }
    test(is_possible_divide);
}
