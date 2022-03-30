//! 统计重复个数

use std::collections::HashMap;

/// 找循环节
pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let len2 = s2.len();
    if n1 == 0 { return 0; }
    let mut s1cnt = 0;
    let mut s2cnt = 0;
    let mut index = 0;
    // recall 是我们用来找循环节的变量，它是一个哈希映射
    // 我们如何找循环节？假设我们遍历了 s1cnt 个 s1，此时匹配到了第 s2cnt 个 s2 中的第 index 个字符
    // 如果我们之前遍历了 s1cnt' 个 s1 时，匹配到的是第 s2cnt' 个 s2 中同样的第 index 个字符，那么就有循环节了
    // 我们用 (s1cnt', s2cnt', index) 和 (s1cnt, s2cnt, index) 表示两次包含相同 index 的匹配结果
    // 那么哈希映射中的键就是 index，值就是 (s1cnt', s2cnt') 这个二元组
    // 循环节就是；
    //    - 前 s1cnt' 个 s1 包含了 s2cnt' 个 s2
    //    - 以后的每 (s1cnt - s1cnt') 个 s1 包含了 (s2cnt - s2cnt') 个 s2
    // 那么还会剩下 (n1 - s1cnt') % (s1cnt - s1cnt') 个 s1, 我们对这些与 s2 进行暴力匹配
    // 注意 s2 要从第 index 个字符开始匹配
    let mut recall = HashMap::new();
    loop {
        // 我们多遍历一个 s1，看看能不能找到循环节
        s1cnt += 1;
        for &ch in s1 {
            if ch == s2[index] {
                index += 1;
                if index == len2 {
                    s2cnt += 1;
                    index = 0;
                }
            }
        }
        // 还没有找到循环节，所有的 s1 就用完了
        if s1cnt == n1 {
            return s2cnt / n2;
        }
        // 出现了之前的 index，表示找到了循环节
        if let Some(&(s1cnt_prime, s2cnt_prime)) = recall.get(&index) {
            // 前 s1cnt' 个 s1 包含了 s2cnt' 个 s2
            // 以后的每 (s1cnt - s1cnt') 个 s1 包含了 (s2cnt - s2cnt') 个 s2
            let every1 = s1cnt - s1cnt_prime;
            let get2 = s2cnt - s2cnt_prime;
            // result 存储的是 S1 包含的 s2 的数量，考虑的之前的 pre_loop 和 in_loop
            let mut result = s2cnt_prime + (n1 - s1cnt_prime) / every1 * get2;
            // S1 的末尾还剩下一些 s1，我们暴力进行匹配
            let rest = (n1 - s1cnt_prime) % every1;
            for _ in 0..rest {
                for &ch in s1 {
                    if ch == s2[index] {
                        index += 1;
                        if index == len2 {
                            result += 1;
                            index = 0;
                        }
                    }
                }
            }
            // S1 包含 result 个 s2，那么就包含 result / n2 个 S2
            return result / n2;
        } else {
            recall.insert(index, (s1cnt, s2cnt));
        }
    }
}

fn main() {
    assert_eq!(get_max_repetitions(String::from("lovelive"), 1, String::from("lovelive"), 10), 0);
    assert_eq!(get_max_repetitions(String::from("acb"), 4, String::from("ab"), 2), 2);
    assert_eq!(get_max_repetitions(String::from("acb"), 1, String::from("acb"), 1), 1);
}
