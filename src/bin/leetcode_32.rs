//! 最长有效括号


/// 找'()' 然后向两边扩展，找最大长度，然后看能不能和前面的接上
pub fn longest_valid_parentheses(s: String) -> i32 {
    if s.len() < 2 { return 0; }
    let mut ans = 0;

    let bytes = s.as_bytes();
    let mut i = 0;
    let n = bytes.len();
    let mut help = std::collections::HashMap::new();
    while i < n - 1 {
        if bytes[i] == b'(' && bytes[i + 1] == b')' {
            // 判断两边
            let mut left = i;
            let mut right = i + 1;

            loop {
                let mut flag = true;
                while left > 0 && right < n - 1 && bytes[left - 1] == b'(' && bytes[right + 1] == b')' {
                    flag = false;
                    left -= 1;
                    right += 1;
                }
                if left > 0 && help.contains_key(&(left - 1)) {
                    let last_length = *help.get(&(left - 1)).unwrap();
                    left -= last_length;
                } else {
                    if flag { break; }
                }
            }
            let len = right - left + 1;
            ans = if len > ans { len } else { ans };
            i = right + 1;
            help.insert(right, len);
        } else {
            i += 1;
        }
    }
    ans as i32
}

/// 双指针，不需要额外空间
/// 从左往右遍历，如果left==right，说明相等，记录最大值。如果right>left，重置为0
/// 对于 (() 的情况，从右往左进行相同的遍历即可
pub fn longest_valid_parentheses2(s: String) -> i32 {
    let (mut left, mut right, mut result) = (0, 0, 0);
    let bytes = s.as_bytes();
    let len = bytes.len();
    for i in 0..len {
        if bytes[i] == b'(' { left += 1; } else { right += 1; }
        if left == right {
            result = result.max(left * 2);
        } else if right >= left {
            left = 0;
            right = 0;
        }
    }
    left = 0;
    right = 0;
    for i in (0..s.len()).rev() {
        if bytes[i] == b'(' { left += 1; } else { right += 1; }
        if left == right {
            result = result.max(left * 2);
        } else if left >= right {
            left = 0;
            right = 0;
        }
    }
    result
}

/// 定义 dp[i] 表示以下标 i 字符结尾的最长有效括号的长度
/// dp[i] = dp[i-1] + dp[i-2-dp[i-1]] + 2 when s[i-1] == b')' and s[i-1-dp[i-1]] == '('
/// dp[i] = dp[i-1] + 2 when dp[i-1] == b'('
pub fn longest_valid_parentheses3(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = 0;
    let mut dp = vec![0; len + 1];
    for i in 1..len {
        if s[i] == b')' {
            if s[i - 1] == b'(' {
                dp[i] = if i > 1 { dp[i - 2] } else { 0 } + 2;
            } else if i > dp[i - 1] && s[i - 1 - dp[i - 1]] == b'(' {
                dp[i] = dp[i - 1] + if i > 1 + dp[i - 1] { dp[i - 2 - dp[i - 1]] } else { 0 } + 2;
            }
            result = result.max(dp[i]);
        }
    }
    result as i32
}

/// 把能匹配的括号标为true，最后看连续的true有多少个
pub fn longest_valid_parentheses4(s: String) -> i32 {
    let mut s = s.into_bytes();
    let mut stk = vec![];
    let len = s.len();
    for i in 0..len {
        if s[i] == b'(' {
            stk.push(i);
        } else if !stk.is_empty() {
            let j = stk.pop().unwrap();
            s[i] = 1;
            s[j] = 1;
        }
    }
    let mut result = 0;
    let mut cnt = 0;
    for num in s {
        if num == 1 {
            cnt += 1;
            result = result.max(cnt);
        } else {
            cnt = 0;
        }
    }
    result
}

/// 始终保持栈底元素为当前已经遍历过的元素中「最后一个没有被匹配的右括号的下标」
pub fn longest_valid_parentheses5(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let mut stk = vec![-1];
    let mut result = 0;
    for i in 0..len {
        if s[i] == b'(' {
            stk.push(i as i32);
        } else {
            stk.pop();
            if stk.is_empty() {
                stk.push(i as i32);
            } else {
                result = result.max(i as i32 - *stk.last().unwrap());
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("(()")), 2);
        assert_eq!(func(String::from(")()())")), 4);
        assert_eq!(func(String::from("")), 0);
        assert_eq!(func(String::from("()")), 2);
        assert_eq!(func(String::from("())")), 2);
        assert_eq!(func(String::from("()(())")), 6);
    }
    test(longest_valid_parentheses);
    test(longest_valid_parentheses2);
    test(longest_valid_parentheses3);
    test(longest_valid_parentheses4);
    test(longest_valid_parentheses5);
}
