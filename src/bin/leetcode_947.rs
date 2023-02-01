//! 移除最多的同行或同列石头

use leetcode::union_set::UnionSetHashMap;

pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let mut us = UnionSetHashMap::new();
    let len = stones.len();
    for stone in stones {
        us.union(stone[0] as usize + 10001, stone[1] as usize);
    }
    return (len - us.count) as i32;
}


fn main() {
    assert_eq!(remove_stones(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 2]]), 5);
    assert_eq!(remove_stones(vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]]), 3);
    assert_eq!(remove_stones(vec![vec![0, 0]]), 0);
}
