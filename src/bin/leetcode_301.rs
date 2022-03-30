//! 删除无效的括号

use std::collections::HashSet;

use leetcode::unorder;

pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let s = s.as_bytes();
    let len = s.len();
    let mut left = 0;
    let mut right = 0;
    for i in 0..len {
        if s[i] == b'(' {
            left += 1;
        } else if s[i] == b')' {
            if left == 0 {
                right += 1;
            } else if left > 0 {
                left -= 1;
            }
        }
    }
    let mut ans = HashSet::new();
    fn dfs(s: &[u8], left_cnt: i32, right_cnt: i32, left: i32, right: i32, idx: usize, cur: &mut Vec<u8>, ans: &mut HashSet<String>) {
        if idx == s.len() {
            if left == 0 && right == 0 {
                unsafe { ans.insert(String::from_utf8_unchecked(cur.to_vec())); }
            }
            return;
        }
        if s[idx] == b'(' && left > 0 {
            dfs(s, left_cnt, right_cnt, left - 1, right, idx + 1, cur, ans);
        }
        if s[idx] == b')' && right > 0 {
            dfs(s, left_cnt, right_cnt, left, right - 1, idx + 1, cur, ans);
        }
        cur.push(s[idx]);
        if s[idx] != b'(' && s[idx] != b')' {
            dfs(s, left_cnt, right_cnt, left, right, idx + 1, cur, ans);
        } else if s[idx] == b'(' {
            dfs(s, left_cnt + 1, right_cnt, left, right, idx + 1, cur, ans);
        } else if right_cnt < left_cnt {
            dfs(s, left_cnt, right_cnt + 1, left, right, idx + 1, cur, ans);
        }
        cur.pop();
    }
    dfs(s, 0, 0, left, right, 0, &mut vec![], &mut ans);
    ans.into_iter().collect()
}

fn main() {
    assert_eq!(unorder(remove_invalid_parentheses(String::from("()())()"))), vec!["(())()", "()()()"]);
    assert_eq!(unorder(remove_invalid_parentheses(String::from("(a)())()"))), vec!["(a())()", "(a)()()"]);
    assert_eq!(unorder(remove_invalid_parentheses(String::from(")("))), vec![""]);
}
