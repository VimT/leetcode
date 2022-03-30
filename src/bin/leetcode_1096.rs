//! 花括号展开 II

use std::collections::{HashSet, VecDeque};

/// bfs遇到括号就展开
pub fn brace_expansion_ii(expression: String) -> Vec<String> {
    let mut q = VecDeque::new();
    q.push_back(expression);
    let mut result = HashSet::new();
    while !q.is_empty() {
        let exp = q.pop_front().unwrap();
        if exp.find('{').is_none() {
            result.insert(exp);
            continue;
        }
        let mut i = 0;
        let mut left = 0;
        let s = exp.as_bytes();
        while s[i] != b'}' {
            if s[i] == b'{' {
                left = i;
            }
            i += 1;
        }
        let right = i;

        let before = exp[0..left].to_string();
        let after = exp[right + 1..].to_string();
        let strs = exp[left + 1..right].split(',');
        for str in strs {
            q.push_back(format!("{}{}{}", before, str, after));
        }
    }
    let mut result: Vec<String> = result.into_iter().collect();
    result.sort_unstable();
    result
}

struct Parser<'a> {
    exp: &'a [u8],
    i: usize,
}

#[derive(Debug)]
struct ParseError;

impl<'a> Parser<'a> {
    fn new(exp: &'a [u8]) -> Self {
        Self { exp, i: 0 }
    }

    fn next(&mut self) -> Result<u8, ParseError> {
        if self.i < self.exp.len() {
            self.i += 1;
            return Ok(self.exp[self.i - 1]);
        }
        Err(ParseError)
    }

    fn chars(&mut self) -> Result<String, ParseError> {
        let mut result = vec![];
        while self.i < self.exp.len() && self.exp[self.i].is_ascii_alphabetic() {
            result.push(self.next()?);
        }
        if result.is_empty() {
            return Err(ParseError);
        }
        Ok(unsafe { String::from_utf8_unchecked(result) })
    }

    // {...} or WORD
    fn item(&mut self) -> Result<HashSet<String>, ParseError> {
        let mut result = HashSet::new();
        result.insert(String::new());
        while self.i < self.exp.len() {
            let mut set = HashSet::new();
            if self.exp[self.i] == b'{' {
                self.next()?;
                set = self.expr()?;
                self.next()?; // ?
            } else if self.exp[self.i].is_ascii_alphabetic() {
                set.insert(self.chars()?);
            } else { break; }
            let mut next = HashSet::new();
            for l in result {
                for r in &set {
                    next.insert(format!("{}{}", l, r));
                }
            }
            result = next;
        }
        Ok(result)
    }

    // {... , ...} or WORD
    fn expr(&mut self) -> Result<HashSet<String>, ParseError> {
        let mut result = HashSet::new();
        while self.i < self.exp.len() {
            for item in self.item()? {
                result.insert(item);
            }
            if self.i < self.exp.len() && self.exp[self.i] == b',' {
                self.next()?;
            } else {
                break;
            }
        }
        Ok(result)
    }
}


/// 递归下降语法分析
/// expr -> item | item ',' expr
/// item -> factor | factor item
/// factor -> WORD | '{' expr '}'
pub fn brace_expansion_ii2(expression: String) -> Vec<String> {
    let mut parser = Parser::new(expression.as_bytes());
    let mut result: Vec<String> = parser.expr().unwrap().into_iter().collect();
    result.sort_unstable();
    result
}

fn main() {
    fn test(func: fn(expression: String) -> Vec<String>) {
        assert_eq!(func(String::from("{a,b}{c,{d,e}}")), vec!["ac", "ad", "ae", "bc", "bd", "be"]);
        assert_eq!(func(String::from("{{a,z},a{b,c},{ab,z}}")), vec!["a", "ab", "ac", "z"]);
    }
    test(brace_expansion_ii);
    test(brace_expansion_ii2);
}
