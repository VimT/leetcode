//! 不可能得到的最短骰子序列

use std::collections::HashSet;

/// 假如我们要得到长度为3的所有序列，那么就需要填三个空格 _ _ _，其中每个空格都包含1~k，才能保证得到所有长度为3的序列。
/// 于是就可以遍历数组，看看有多少段完全包含1~k的子数组，凑不出来的序列长度在此基础上加1即可。
pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
    let mut set = HashSet::new();
    let mut result = 1;
    for roll in rolls {
        set.insert(roll);
        if set.len() == k as usize {
            result += 1;
            set.clear();
        }
    }
    result
}

fn main() {
    fn test(func: fn(rolls: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![4, 2, 1, 2, 3, 3, 2, 4, 1], 4), 3);
        assert_eq!(func(vec![1, 1, 2, 2], 2), 2);
        assert_eq!(func(vec![1, 1, 3, 2, 2, 2, 3, 3], 4), 1);
    }
    test(shortest_sequence);
}
