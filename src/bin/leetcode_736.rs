//! Lisp 语法解析

use std::collections::HashMap;

struct LispInterpreter<'a> {
    i: usize,
    expr: &'a str,
    stack: Vec<HashMap<&'a str, i32>>,
}

impl<'a> LispInterpreter<'a> {
    fn new(expr: &'a str) -> Self {
        Self { expr, i: 0, stack: vec![HashMap::new()] }
    }

    fn preview(&self, n: usize) -> &'a str {
        &self.expr[self.i..self.i + n]
    }

    fn next(&mut self, n: usize) -> &'a str {
        let result = &self.expr[self.i..self.i + n];
        self.i += n;
        result
    }

    fn next_word(&mut self) -> &'a str {
        let mut end = self.i + 1;
        let s = self.expr.as_bytes();
        while end < s.len() && s[end] != b' ' && s[end] != b')' { end += 1; }
        let word = &self.expr[self.i..end];
        self.i = end;
        word
    }

    fn set_var(&mut self, var_name: &'a str, val: i32) {
        self.stack.last_mut().unwrap().insert(var_name, val);
    }

    fn get_var(&self, var_name: &str) -> i32 {
        for map in self.stack.iter().rev() {
            if let Some(v) = map.get(var_name) {
                return *v;
            }
        }
        unreachable!()
    }

    fn expr(&mut self) -> i32 {
        let next = self.preview(1);
        return if next == "(" {
            self.stack.push(HashMap::new());
            self.next(1);
            let next_word = self.next_word();
            self.next(1); // space
            let result = match next_word {
                "let" => self.let_(),
                "mult" => self.multi(),
                "add" => self.add(),
                _ => unreachable!(),
            };
            self.next(1); //)
            self.stack.pop().unwrap();
            result
        } else if next.as_bytes()[0].is_ascii_lowercase() {
            let var_name = self.next_word();
            self.get_var(var_name)
        } else {
            let num_str = self.next_word();
            num_str.parse().unwrap()
        };
    }

    fn multi(&mut self) -> i32 {
        let lhs = self.expr();
        self.next(1);
        let rhs = self.expr();
        return lhs * rhs;
    }

    fn add(&mut self) -> i32 {
        let lhs = self.expr();
        self.next(1);
        let rhs = self.expr();
        return lhs + rhs;
    }

    fn let_(&mut self) -> i32 {
        loop {
            // if expr?
            if self.preview(1) == "(" {
                return self.expr();
            }
            let var_name = self.next_word();
            let next = self.next(1); // space
            if next == ")" {
                self.i -= var_name.len() + 1; // back
                return self.expr();
            }
            let val = self.expr();
            self.set_var(var_name, val);
            self.next(1); // space
        }
    }
}

pub fn evaluate(expression: String) -> i32 {
    let mut interpreter = LispInterpreter::new(expression.as_str());
    interpreter.expr()
}

fn main() {
    assert_eq!(evaluate(String::from("(let x 7 -12)")), -12);
    assert_eq!(evaluate(String::from("(let x 2 (mult x (let x 3 y 4 (add x y))))")), 14);
    assert_eq!(evaluate(String::from("(let x 3 x 2 x)")), 2);
    assert_eq!(evaluate(String::from("(let x 1 y 2 x (add x y) (add x y))")), 5);
    assert_eq!(evaluate(String::from("(let a1 3 b2 (add a1 1) b2) ")), 4);
    assert_eq!(evaluate(String::from("(let x 2 (add (let x 3 (let x 4 x)) x))")), 6);
    assert_eq!(evaluate(String::from("(let x 2 (mult x 5))")), 10);
}
