//! 得到山形数组的最少删除次数

use std::collections::BTreeMap;

pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let mut tree = BTreeMap::new();
    let len = nums.len();
    let mut up_num = vec![1; len];
    let mut down_num = vec![1; len];
    for i in 0..len {
        tree.insert(nums[i], i);
        for (_, v) in tree.range(0..nums[i]) {
            up_num[i] = up_num[i].max(up_num[*v] + 1);
        }
    }
    tree.clear();
    for i in (0..len).rev() {
        tree.insert(nums[i], i);
        for (_, v) in tree.range(0..nums[i]) {
            down_num[i] = down_num[i].max(down_num[*v] + 1);
        }
    }
    let mut result = i32::MAX;
    for i in 0..len {
        if up_num[i] > 1 && down_num[i] > 1 {
            result = result.min(len as i32 - up_num[i] - down_num[i] + 1);
        }
    }
    result
}

fn main() {
    assert_eq!(minimum_mountain_removals(vec![1, 3, 1]), 0);
    assert_eq!(minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]), 3);
    assert_eq!(minimum_mountain_removals(vec![4, 3, 2, 1, 1, 2, 3, 1]), 4);
    assert_eq!(minimum_mountain_removals(vec![1, 2, 3, 4, 4, 3, 2, 1]), 1);
}
