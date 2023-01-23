//! 串联字符串的最大长度

use leetcode::svec;

pub fn max_length(arr: Vec<String>) -> i32 {
    let arr: Vec<i32> = arr.into_iter().filter_map(|s| {
        let mut num: i32 = 0;
        for &ch in s.as_bytes() {
            num |= 1 << (ch - b'a');
        }
        if num.count_ones() as usize == s.len() {
            Some(num)
        } else { None }
    }).collect();
    let mut result = 0;
    let len = arr.len();
    'out: for i in 1..1 << len {
        let mut cur = 0;
        for j in 0..len {
            if i >> j & 1 == 1 {
                if cur & arr[j] == 0 {
                    cur |= arr[j];
                } else { continue 'out; }
            }
        }
        result = result.max(cur.count_ones());
    }
    result as i32
}

pub fn max_length2(arr: Vec<String>) -> i32 {
    let arr: Vec<i32> = arr.into_iter().filter_map(|s| {
        let mut num: i32 = 0;
        for &ch in s.as_bytes() {
            num |= 1 << (ch - b'a');
        }
        if num.count_ones() as usize == s.len() {
            Some(num)
        } else { None }
    }).collect();
    fn dfs(arr: &Vec<i32>, i: usize, mask: i32, result: &mut i32) {
        if i == arr.len() {
            (*result) = (*result).max(mask.count_ones() as i32);
            return;
        }
        dfs(arr, i + 1, mask, result);
        if mask & arr[i] == 0 {
            dfs(arr, i + 1, mask | arr[i], result);
        }
    }
    let mut result = 0;
    dfs(&arr, 0, 0, &mut result);
    result
}

fn main() {
    fn test(func: fn(arr: Vec<String>) -> i32) {
        assert_eq!(func(svec!["aa", "bb"]), 0);
        assert_eq!(func(svec!["un","iq","ue"]), 4);
        assert_eq!(func(svec!["cha","r","act","ers"]), 6);
        assert_eq!(func(svec!["abcdefghijklmnopqrstuvwxyz"]), 26);
    }
    test(max_length);
    test(max_length2);
}
