//! 公司命名

use std::collections::HashMap;
use leetcode::svec;

/// 定义 cnt[i][j] 表示组中首字母不包含 i 但包含 j 的组的个数。枚举每个组，统计cnt，同时枚举该组的首字母 i 和不在该组的首字母 j，答案即为cnt[i][j] 的累加值。
/// 简单来说就是「有 i 无 j」可以和「无 i 有 j」的字符串互换。
pub fn distinct_names(ideas: Vec<String>) -> i64 {
    let mut map: HashMap<String, u32> = HashMap::new();
    for idea in &ideas {
        let ch_idx = (idea.as_bytes()[0] - b'a') as usize;
        *map.entry(idea[1..].to_string()).or_default() |= 1 << ch_idx;
    }
    let mut result = 0;
    let mut cnt = [[0; 26]; 26];
    for (_, mask) in map {
        for i in 0..26 {
            if mask >> i & 1 == 0 {
                for j in 0..26 {
                    if mask >> j & 1 == 1 {
                        cnt[i][j] += 1;
                    }
                }
            } else {
                for j in 0..26 {
                    if mask >> j & 1 == 0 {
                        result += cnt[i][j];
                    }
                }
            }
        }
    }
    result * 2
}

fn main() {
    fn test(func: fn(ideas: Vec<String>) -> i64) {
        assert_eq!(func(svec!["coffee","donuts","time","toffee"]), 6);
        assert_eq!(func(svec!["lack","back"]), 0);
    }
    test(distinct_names);
}
