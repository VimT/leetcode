//! 戳印序列

use std::collections::VecDeque;

/// 反向思维 : 从 target 反向到初始状态
/// 那么把每一个长度为 stamp 的窗口作为一个节点，把其中不能和 stamp 匹配的字符个数作为该节点的入度
/// 拓扑排序
pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
    let s = stamp.as_bytes();
    let mut t = target.into_bytes();
    let mut result = vec![];
    let mut q = VecDeque::new();
    let mut ind = vec![0; t.len() - s.len() + 1];
    // dis[i][j] 表示t从i开始的 第j个字符是否匹配
    let mut dis = vec![vec![false; s.len()]; t.len() - s.len() + 1];
    for i in 0..=t.len() - s.len() {
        let mut node_ind = s.len();
        for si in 0..s.len() {
            if s[si] == t[si + i] {
                dis[i][si] = true;
                node_ind -= 1;
            }
        }
        ind[i] = node_ind;
        if node_ind == 0 {
            q.push_back(i);
        }
    }
    let mut change = 0;
    while !q.is_empty() && change < t.len() {
        let start = q.pop_front().unwrap();
        result.push(start as i32);
        for si in 0..s.len() {
            if t[start + si] != b'#' {
                t[start + si] = b'#';
                change += 1;
                for back in 0..s.len() {
                    if start + si < back { break; }
                    let bstart = start + si - back;
                    if bstart >= dis.len() { continue; }
                    if !dis[bstart][back] {
                        dis[bstart][back] = true;
                        ind[bstart] -= 1;
                        if ind[bstart] == 0 {
                            q.push_back(bstart);
                        }
                    }
                }
            }
        }
    }
    if change != t.len() { return vec![]; }
    result.reverse();
    result
}

fn main() {
    assert_eq!(moves_to_stamp(String::from("mda"), String::from("mdadddaaaa")), vec![]);
    assert_eq!(moves_to_stamp(String::from("abc"), String::from("ababc")), vec![0, 2]);
    assert_eq!(moves_to_stamp(String::from("abca"), String::from("aabcaca")), vec![3, 0, 1]);
}
