//! 括号生成

use leetcode::{svec, unorder};

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn inner(stack: &mut Vec<u8>, open: usize, close: usize, max: usize, ans: &mut Vec<String>) {
        if stack.len() == max * 2 {
            ans.push(unsafe { String::from_utf8_unchecked(stack.clone()) });
            return;
        }
        if open < max {
            stack.push(b'(');
            inner(stack, open + 1, close, max, ans);
            stack.pop();
        }
        if close < open {
            stack.push(b')');
            inner(stack, open, close + 1, max, ans);
            stack.pop();
        }
    }
    let mut ans = vec![];
    inner(&mut vec![], 0, 0, n as usize, &mut ans);
    ans
}

/// dp[n] = "( {dp[i] } ) {dp[n-1-i]}"
pub fn generate_parenthesis_dp(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut dp = vec![];
    dp.push(vec!["".to_string()]);
    dp.push(vec!["()".to_string()]);
    for i in 2..=n {
        let mut l = vec![];
        for j in 0..i {
            for k1 in &dp[j] {
                for k2 in &dp[i - 1 - j] {
                    l.push(["(", k1, ")", k2].concat());
                }
            }
        }
        dp.push(l);
    }
    dp[n].clone()
}

fn main() {
    fn test(func: fn(n: i32) -> Vec<String>) {
        assert_eq!(unorder(func(3)), unorder(svec!["((()))", "(()())", "(())()", "()(())", "()()()"]));
        assert_eq!(unorder(func(1)), unorder(svec!["()"]));
    }
    test(generate_parenthesis);
    test(generate_parenthesis_dp);
}
