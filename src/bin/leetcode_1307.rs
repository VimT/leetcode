//! 口算难题

use std::collections::HashSet;
use leetcode::svec;

fn conv(s: &[u8], m: &[i32; 26]) -> i32 {
    let mut num = 0;
    for &ch in s {
        num = num * 10 + m[(ch - b'A') as usize];
    }
    num
}

pub fn is_solvable(words: Vec<String>, result: String) -> bool {
    let mut cm = [-1; 26];
    let mut chars = vec![];
    let mut exist = [false; 26];
    let left_max_len = words.iter().map(|x| x.len()).max().unwrap();
    let right_len = result.len();
    if left_max_len > right_len || left_max_len + 2 < right_len {
        return false;
    }
    let mut ss: Vec<&[u8]> = words.iter().map(|x| x.as_bytes()).collect();
    ss.insert(0, result.as_bytes());
    let max_len = ss.iter().map(|x| x.len()).max().unwrap();
    for i in 0..max_len {
        for s in &ss {
            if i < s.len() {
                let idx = (s[s.len() - 1 - i] - b'A') as usize;
                if !exist[idx] {
                    chars.push(idx);
                    exist[idx] = true;
                }
            }
        }
    }
    let mut cant_zero = [false; 26];
    for s in ss {
        if s.len() > 1 {
            cant_zero[(s[0] - b'A') as usize] = true;
        }
    }

    fn dfs(words: &Vec<&[u8]>, result: &[u8], cant_zero: &[bool; 26], chars: &Vec<usize>, i: usize, left_num: &mut HashSet<i32>, cm: &mut [i32; 26]) -> bool {
        if chars.len() == i {
            return words.iter().map(|x| conv(x, cm)).sum::<i32>() == conv(result, cm);
        }

        // 剪枝验证
        let mut carry = None;
        let max_len = words.iter().map(|x| x.len()).min().unwrap().min(result.len());
        'out: for i in 0..max_len {
            let should_sum = if i < result.len() {
                let rch = result[result.len() - 1 - i];
                let n = cm[(rch - b'A') as usize];
                if n == -1 {
                    carry = None;
                    continue 'out;
                }
                n
            } else {
                0
            };

            let mut wei_sum = 0;
            for word in words {
                if i < word.len() {
                    let ww = word[word.len() - 1 - i];
                    let n = cm[(ww - b'A') as usize];
                    if n == -1 {
                        carry = None;
                        continue 'out;
                    }
                    wei_sum += n;
                }
            }
            if let Some(carry) = carry {
                wei_sum += carry;
                if should_sum != wei_sum % 10 {
                    return false;
                }
            } else {
                if should_sum != wei_sum % 10 && should_sum != (wei_sum + 1) % 10 {
                    return false;
                }
            }
            carry = Some(wei_sum / 10);
        }

        let can_use: Vec<i32> = left_num.iter().cloned().collect();
        for num in can_use {
            if num == 0 && cant_zero[chars[i]] { continue; }
            cm[chars[i]] = num;
            left_num.remove(&num);
            if dfs(words, result, cant_zero, chars, i + 1, left_num, cm) { return true; }
            cm[chars[i]] = -1;
            left_num.insert(num);
        }
        false
    }
    let words = words.iter().map(|x| x.as_bytes()).collect();
    dfs(&words, result.as_bytes(), &cant_zero, &chars, 0, &mut (0..10i32).collect(), &mut cm)
}

/// https://leetcode.cn/problems/verbal-arithmetic-puzzle/solution/suan-nan-ti-by-leetcode-solution/
/// 多项式合并  SEND + MORE = MONEY
/// SEND  =             S * 1000 + E * 100 + N * 10 + D
/// MORE  =             M * 1000 + O * 100 + R * 10 + E
/// MONEY = M * 10000 + O * 1000 + N * 100 + E * 10 + Y
/// M * (-9000) + S * 1000 + O * (-900) + E * 91 + N * (-90) + R * 10 + D + Y * (-1) = 0
/// 这样 根据多项式系数大的优先考虑，-9000 那么M应该很小
pub fn is_solvable2(words: Vec<String>, result: String) -> bool {
    let left_max_len = words.iter().map(|x| x.len()).max().unwrap();
    let right_len = result.len();
    if left_max_len > right_len || left_max_len + 2 < right_len {
        return false;
    }
    let mut weight = vec![0; 26];
    let mut cant_zero = vec![false; 26];
    for s in &words {
        let s = s.as_bytes();
        for (i, &ch) in s.iter().enumerate() {
            weight[(ch - b'A') as usize] += 10i32.pow((s.len() - i - 1) as u32);
        }
        if s.len() > 1 {
            cant_zero[(s[0] - b'A') as usize] = true;
        }
    }
    for (i, &ch) in result.as_bytes().iter().enumerate() {
        weight[(ch - b'A') as usize] -= 10i32.pow((result.len() - i - 1) as u32);
    }
    if result.len() > 1 {
        cant_zero[(result.as_bytes()[0] - b'A') as usize] = true;
    }
    // ch, 系数，后面的最小值，后面的最大值
    let mut weight: Vec<(usize, i32, i32, i32, i32)> = weight.iter().enumerate().filter(|x| *x.1 != 0).map(|x| (x.0, *x.1, 0, 0, 0)).collect();
    weight.sort_unstable_by_key(|x| -(x.1.abs()));
    let len = weight.len();
    for w in &mut weight {
        if cant_zero[w.0] {
            w.4 = 1;
        }
    }
    for i in 0..len {
        let mut min = 0;
        let mut max = 0;
        let mut max_s1 = 9;
        let mut max_s2 = 0;
        let mut min_s1 = 9;
        let mut min_s2 = 0;
        for j in i..len {
            if weight[j].1 < 0 {
                min += min_s1 * weight[j].1;
                min_s1 -= 1;
                max += max_s2 * weight[j].1;
                max_s2 += 1;
            } else {
                min += min_s2 * weight[j].1;
                min_s2 += 1;
                max += max_s1 * weight[j].1;
                max_s1 -= 1;
            }
        }
        weight[i].2 = min;
        weight[i].3 = max;
    }

    fn dfs(weight: &Vec<(usize, i32, i32, i32, i32)>, i: usize, cur: i32, used: &mut [bool; 10]) -> bool {
        if i == weight.len() {
            return cur == 0;
        }
        // 当前+后面最小的 都会 > 0
        if cur + weight[i].2 > 0 || cur + weight[i].3 < 0 {
            return false;
        }
        for j in weight[i].4..10 {
            if !used[j as usize] {
                used[j as usize] = true;
                if dfs(weight, i + 1, cur + weight[i].1 * j, used) { return true; }
                used[j as usize] = false;
            }
        }
        false
    }
    dfs(&weight, 0, 0, &mut [false; 10])
}

fn main() {
    fn test(func: fn(words: Vec<String>, result: String) -> bool) {
        assert_eq!(func(svec!["SEND","MORE"], String::from("MONEY")), true);
        assert_eq!(func(svec!["CBA","CBA","CBA","CBA","CBA"], String::from("EDD")), false);
        assert_eq!(func(svec!["A", "B"], String::from("A")), true);
        assert_eq!(func(svec!["SIX","SEVEN","SEVEN"], String::from("TWENTY")), true);
        assert_eq!(func(svec!["LEET","CODE"], String::from("POINT")), false);
    }
    test(is_solvable);
    test(is_solvable2);
}
