//! 基本计算器 II

use std::collections::{HashMap, VecDeque};

enum Cal {
    Num(i32),
    Operator(char),
}

/// 基本计算器，根据栈的特性瞎写的
pub fn calculate(s: String) -> i32 {
    let mut stack: VecDeque<Cal> = VecDeque::new();
    let mut current_nums = vec![];
    let mut words = vec![];
    let mut ans = 0;

    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        let c = chars[i];
        if c.is_whitespace() { continue; }
        if c.is_ascii_digit() {
            current_nums.push(c);
        } else {
            words.push(Cal::Num(current_nums.iter().clone().collect::<String>().parse::<i32>().unwrap()));
            current_nums.clear();
            words.push(Cal::Operator(c));
        }
    }
    words.push(Cal::Num(current_nums.into_iter().collect::<String>().parse::<i32>().unwrap()));
    for i in words {
        match i {
            // 当前是数字时，看顶部的操作符，如果是*/就计算并push
            Cal::Num(right) => {
                if stack.is_empty() {
                    stack.push_back(Cal::Num(right));
                    continue;
                }
                if let Cal::Operator(op) = stack.pop_back().unwrap() {
                    if op == '*' || op == '/' {
                        if let Cal::Num(left) = stack.pop_back().unwrap() {
                            stack.push_back(Cal::Num(if op == '*' { left * right } else { left / right }));
                        }
                    } else {
                        if op == '+' {
                            stack.push_back(Cal::Num(right));
                        } else {
                            stack.push_back(Cal::Num(-right));
                        }
                    }
                }
            }
            Cal::Operator(o) => {
                stack.push_back(Cal::Operator(o));
            }
        }
    }

    for i in stack {
        match i {
            Cal::Num(v) => ans += v,
            Cal::Operator(_v) => panic!("should not be here")
        }
    }
    ans
}

/// 中缀转后缀表达式  5+2*2 => 522*+ ，再根据后缀表达式求结果方法求结果。还有带括号版本的，需要遇到')'的时候才弹出stack直到'('
/// 另一种转化方法：
/// 1)先按照运算符的优先级对中缀表达式加括号，变成( ( a+(b*c) ) + ( ((d*e)+f) *g ) )
/// 2)将运算符移到括号的后面，变成((a(bc)*)+(((de)*f)+g)*)+
/// 3)去掉括号，得到abc*+de*f+g*+
pub fn calculate_postfix(s: String) -> i32 {
    //栈用来存符号,为了确定优先级
    let mut stack = VecDeque::new();
    //后缀表达式结果
    let mut postfix_expression = vec![];
    let level: HashMap<char, i32> = vec![('*', 1), ('/', 1), ('+', 0), ('-', 0)].into_iter().collect();
    let mut cur = 0;
    for i in s.chars() {
        if i.is_ascii_whitespace() { continue; }
        if i.is_ascii_digit() {
            cur = cur * 10 + (i as u8 - b'0') as i32;
        } else {
            stack.push_back(Cal::Num(cur));
            cur = 0;
            while !stack.is_empty() {
                let back = stack.back().unwrap();
                if let Cal::Operator(op) = back {
                    if level[op] < level[&i] {
                        break;
                    }
                }
                postfix_expression.push(stack.pop_back().unwrap());
            }
            stack.push_back(Cal::Operator(i));
        }
    }
    stack.push_back(Cal::Num(cur));
    for _ in 0..stack.len() {
        postfix_expression.push(stack.pop_back().unwrap())
    }
    let mut stack: VecDeque<i32> = VecDeque::new();
    for i in postfix_expression {
        match i {
            Cal::Num(num) => {
                stack.push_back(num);
            }
            Cal::Operator(op) => {
                let right = stack.pop_back().unwrap();
                let left = stack.pop_back().unwrap();
                let v = match op {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    _ => { panic!("error operator") }
                };
                stack.push_back(v);
            }
        }
    }
    return stack.pop_back().unwrap();
}

pub fn calculate_single_stack(s: String) -> i32 {
    let mut s = s.as_bytes().to_vec();
    s.push(b'?');
    let mut stack = vec![];
    let mut cur = 0;
    let mut flag = b'+';
    for i in 0..s.len() {
        if s[i].is_ascii_whitespace() { continue; }
        if s[i].is_ascii_digit() {
            cur = cur * 10 + (s[i] - b'0') as i32;
        } else {
            match flag {
                b'+' => stack.push(cur),
                b'-' => stack.push(-cur),
                b'*' => {
                    let pop = stack.pop().unwrap();
                    stack.push(pop * cur);
                }
                b'/' => {
                    let pop = stack.pop().unwrap();
                    stack.push(pop / cur);
                }
                _ => panic!()
            }
            flag = s[i];
            cur = 0;
        }
    }
    let mut result = 0;
    while !stack.is_empty() {
        result += stack.pop().unwrap();
    }
    result
}

/// 指针法，通过4个记录器。。。单栈简化版本
pub fn calculate_ptr(mut s: String) -> i32 {
    // 存 */ 之前,前面的那个数
    let mut pre = 0;
    // left，在更新之前，就是 操作符前面的数。 更新之后就是即将要操作的值
    let mut left = 0;
    // right，决定了下一个值
    let mut right: i32 = 0;
    // 上一个符号
    let mut sign: char = '+';
    s.push_str("?");
    let to_i32 = |x: char| { (x as u8 - b'0') as i32 };
    for c in s.chars() {
        if c.is_ascii_whitespace() { continue; }
        if c.is_ascii_digit() {
            right = 10 * right + to_i32(c);
        } else {
            match sign {
                '+' => {
                    pre += left;
                    left = right;
                }
                '-' => {
                    pre += left;
                    left = -right;
                }
                '*' => left *= right,
                '/' => left /= right,
                _ => ()
            }
            right = 0;
            sign = c;
        }
    }
    return pre + left;
}

pub fn calculate_recursion(s: String) -> i32 {
    fn cal(s: &[u8]) -> i32 {
        let mut len = s.len();
        for i in (0..len).rev() {
            if s[i] == b'-' {
                return cal(&s[0..i]) - cal(&s[i + 1..]);
            } else if s[i] == b'+' {
                return cal(&s[0..i]) + cal(&s[i + 1..]);
            }
        }
        for i in (0..len).rev() {
            if s[i] == b'*' {
                return cal(&s[0..i]) * cal(&s[i + 1..]);
            } else if s[i] == b'/' {
                return cal(&s[0..i]) / cal(&s[i + 1..]);
            }
        }
        let mut sum = 0;
        let mut start = 0;
        while s[start] == b' ' { start += 1; }
        while s[len - 1] == b' ' { len -= 1; }
        for i in start..len {
            sum = 10 * sum + (s[i] - b'0') as i32;
        }
        sum
    }
    cal(s.as_bytes())
}


fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("3+2*2")), 7);
        assert_eq!(func(String::from(" 3/2 ")), 1);
        assert_eq!(func(String::from(" 3+5 / 2 ")), 5);
    }
    test(calculate);
    test(calculate_postfix);
    test(calculate_ptr);
    test(calculate_recursion);
    test(calculate_single_stack);
}
