//! 逆波兰表达式求值

use leetcode::svec;

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = std::collections::VecDeque::new();

    for i in tokens {
        if let Ok(v) = i.parse::<i32>() {
            stack.push_back(v);
        } else {
            let right = stack.pop_back().unwrap();
            let left = stack.pop_back().unwrap();
            match i.as_str() {
                "+" => stack.push_back(left + right),
                "-" => stack.push_back(left - right),
                "*" => stack.push_back(left * right),
                "/" => stack.push_back(left / right),
                _ => unimplemented!()
            };
        }
    }
    return stack.pop_back().unwrap();
}


fn main() {
    assert_eq!(eval_rpn(svec!["2","1","+","3","*"]), 9);
    assert_eq!(eval_rpn(svec!["4","13","5","/","+"]), 6);
    assert_eq!(eval_rpn(svec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"]), 22);
}
