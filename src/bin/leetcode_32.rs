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

/// left right 两个指针，左到右扫一遍，右到左再扫一遍
pub fn longest_valid_parentheses_double_point(s: String) -> i32 {
    let (mut left, mut right, mut ans) = (0, 0, 0);
    let bytes = s.as_bytes();
    let len = bytes.len();
    for i in 0..len {
        if bytes[i] == b'(' { left += 1; } else { right += 1; }
        if left == right {
            ans = if ans > 2 * right { ans } else { 2 * right };
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
            ans = if ans > 2 * left { ans } else { 2 * left };
        } else if left >= right {
            left = 0;
            right = 0;
        }
    }
    ans
}


fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("(()")), 2);
        assert_eq!(func(String::from(")()())")), 4);
        assert_eq!(func(String::from("")), 0);
    }
    test(longest_valid_parentheses);
    test(longest_valid_parentheses_double_point);
}
