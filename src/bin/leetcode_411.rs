//! 最短独占单词缩写

use std::fmt::Debug;
use leetcode::svec;

/// 把dict里相同长度的 单词，相同字母的位置 置1，方便后面二进制运算
/// 所有可能的情况列出来，按长度排序
pub fn min_abbreviation(target: String, dictionary: Vec<String>) -> String {
    let s = target.as_bytes();
    let len = s.len();
    let dict: Vec<i32> = dictionary.iter().filter(|x| x.len() == len).map(|x| {
        let x = x.as_bytes();
        let mut num = 0;
        for i in 0..len {
            if s[i] == x[i] {
                num |= 1 << i;
            }
        }
        num
    }).collect();

    let mut all = vec![];
    for mask in 0..1 << len {
        let mut num = 0;
        let mut result_len = 0;
        for i in 0..len {
            if mask >> i & 1 == 1 {
                result_len += if num >= 10 { 2 } else if num > 0 { 1 } else { 0 };
                num = 0;
                result_len += 1;
            } else {
                num += 1;
            }
        }
        result_len += if num >= 10 { 2 } else if num > 0 { 1 } else { 0 };
        all.push((result_len, mask));
    }
    all.sort_unstable();
    for (_, mask) in all {
        let mut ok = true;
        for &word in &dict {
            if mask & word == mask {
                // 说明 此mask情况，dict里的也可以做到
                ok = false;
                break;
            }
        }
        if ok {
            let mut result = vec![];
            let mut num = 0;
            for i in 0..len {
                if mask >> i & 1 == 1 {
                    if num > 0 { result.extend_from_slice(num.to_string().as_bytes()); }
                    result.push(s[i]);
                    num = 0;
                } else {
                    num += 1;
                }
            }
            if num > 0 { result.extend_from_slice(num.to_string().as_bytes()); }
            return unsafe { String::from_utf8_unchecked(result) };
        }
    }
    target
}

/// 不使用二进制遍历，使用dfs，方便剪枝
pub fn min_abbreviation_dfs(target: String, dictionary: Vec<String>) -> String {
    let s = target.as_bytes();
    let len = s.len();
    let dict: Vec<i32> = dictionary.iter().filter(|x| x.len() == len).map(|x| {
        let x = x.as_bytes();
        let mut num = 0;
        for i in 0..len {
            if s[i] == x[i] {
                num |= 1 << i;
            }
        }
        num
    }).collect();

    fn check(mask: i32, dict: &Vec<i32>) -> bool {
        for &word in dict {
            if mask & word == mask {
                // 说明 此mask情况，dict里的也可以做到
                return false;
            }
        }
        true
    }
    if check(0, &dict) {
        return s.len().to_string();
    }

    fn dfs(dict: &Vec<i32>, len: usize, idx: usize, mask: i32, cur_len: i32, min_len: &mut i32, result: &mut i32) {
        if idx == len {
            return;
        }
        // 长度计算放在dfs里
        let mut last_is_char_len = cur_len;
        let prev_is_char = if idx > 0 { mask >> (idx - 1) & 1 == 1 } else { true };
        if idx == len - 1 {
            if !prev_is_char {
                // 最后一位是数字
                last_is_char_len += 1;
            }
        } else {
            if prev_is_char {
                // 前一位是字母
                last_is_char_len += 1;
            } else {
                // 前一位是数字，长度是数字+字母
                last_is_char_len += 2;
            }
        }
        // 剪枝，长度小于之前的有效长度
        if last_is_char_len < *min_len {
            if check(mask | 1 << idx, dict) {
                *result = mask | 1 << idx;
                *min_len = last_is_char_len;
                return;
            }
            dfs(dict, len, idx + 1, mask | 1 << idx, last_is_char_len, min_len, result);
        }
        dfs(dict, len, idx + 1, mask, cur_len, min_len, result)
    }

    let mut mask = 0;
    let mut min_len = i32::MAX;
    dfs(&dict, len, 0, 0, 0, &mut min_len, &mut mask);

    let mut result = vec![];
    let mut num = 0;
    for i in 0..len {
        if mask >> i & 1 == 1 {
            if num > 0 { result.extend_from_slice(num.to_string().as_bytes()); }
            result.push(s[i]);
            num = 0;
        } else {
            num += 1;
        }
    }
    if num > 0 { result.extend_from_slice(num.to_string().as_bytes()); }
    return unsafe { String::from_utf8_unchecked(result) };
}

fn main() {
    fn test(func: fn(target: String, dictionary: Vec<String>) -> String) {
        fn is_one_of<T: Eq + Debug>(one: T, all: Vec<T>) {
            for val in &all {
                if val == &one {
                    return;
                }
            }
            panic!("{:?} is not one of {:?}", one, all);
        }
        is_one_of(func(String::from("leetcode"), svec![""]), svec!["8"]);
        is_one_of(func(String::from("leetcode"), svec!["lyftcode","leetcold","litecode","lietcode","leetccod","lyftcold"]), svec!["1e5e", "le5e"]);
        is_one_of(func(String::from("apple"), svec!["blade"]), svec!["a4"]);
        is_one_of(func(String::from("apple"), svec!["blade","plain","amber"]), svec!["ap3", "1p3"]);
    }
    test(min_abbreviation);
    test(min_abbreviation_dfs);
}
