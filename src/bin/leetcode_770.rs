//! 基本计算器 IV

use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

use leetcode::svec;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Item<'a> {
    num: i32,
    var_list: Vec<&'a str>,
}


impl<'a> Mul for Item<'a> {
    type Output = Item<'a>;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        self.var_list.append(&mut rhs.var_list);
        self.var_list.sort_unstable();
        Item { num: self.num * rhs.num, var_list: self.var_list }
    }
}

impl<'a> Item<'a> {
    fn from_str(s: &'a [u8], map: &HashMap<&str, i32>) -> Self {
        let mut num = 1;
        let mut var_list = vec![];
        let ss = unsafe { std::str::from_utf8_unchecked(s) };
        if s[0].is_ascii_digit() {
            num *= ss.parse::<i32>().unwrap();
        } else if let Some(val) = map.get(ss) {
            num = *val;
        } else {
            var_list.push(ss);
        }
        Item { num, var_list }
    }

    fn neg(&mut self) {
        self.num = -self.num;
    }

    fn to_string(&self) -> Option<String> {
        if self.num == 0 { return None; }
        if self.var_list.is_empty() { return Some(self.num.to_string()); } else {
            Some(format!("{}*{}", self.num, self.var_list.join("*")))
        }
    }
}

struct Expressions<'a>(Vec<Item<'a>>);

impl<'a> Expressions<'a> {
    fn cal(self) -> Expressions<'a> {
        let mut num = 0;
        let mut var_items = HashMap::new();
        for item in &self.0 {
            if item.var_list.is_empty() {
                num += item.num;
            } else {
                *var_items.entry(&item.var_list).or_insert(0i32) += item.num;
            }
        }
        let mut result: Vec<Item> = var_items.into_iter().map(|(k, v)| Item { num: v, var_list: k.clone() }).collect();
        result.sort_unstable_by_key(|x| (-(x.var_list.len() as i32), x.var_list.clone()));
        result.push(Item { num, var_list: vec![] });
        Expressions(result)
    }
}

impl<'a> Add for Expressions<'a> {
    type Output = Expressions<'a>;

    fn add(self, mut rhs: Self) -> Self::Output {
        if self.0.is_empty() { return rhs; }
        let mut items = self.0;
        items.append(&mut rhs.0);
        Expressions(items).cal()
    }
}

impl<'a> Sub for Expressions<'a> {
    type Output = Expressions<'a>;

    fn sub(self, mut rhs: Self) -> Self::Output {
        for item in &mut rhs.0 {
            item.neg();
        }
        self.add(rhs)
    }
}

impl<'a> Mul for Expressions<'a> {
    type Output = Expressions<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Vec::with_capacity(self.0.len() * rhs.0.len());
        for left_item in self.0 {
            for right_item in &rhs.0 {
                result.push(left_item.clone() * right_item.clone());
            }
        }
        Expressions(result).cal()
    }
}


pub fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
    fn dfs<'a>(s: &'a [u8], i: &mut usize, map: &HashMap<&'a str, i32>) -> Expressions<'a> {
        let mut stack = vec![];
        let mut prev_op = b'+';
        let mut last_expr = Expressions(vec![]);

        while *i < s.len() {
            if s[*i] == b'(' {
                *i += 1;
                last_expr = dfs(s, i, map);
            } else if s[*i] == b' ' {
                *i += 1;
            } else if matches!(s[*i], b'+' | b'-' | b'*' | b'$' | b')') {
                match prev_op {
                    b'+' => { stack.push(last_expr) }
                    b'-' => {
                        for item in &mut last_expr.0 {
                            item.neg();
                        };
                        stack.push(last_expr);
                    }
                    b'*' => {
                        let prev = stack.pop().unwrap();
                        stack.push(last_expr * prev);
                    }
                    _ => unreachable!(),
                }
                last_expr = Expressions(vec![]);
                prev_op = s[*i];
                *i += 1;
                if prev_op == b')' {
                    break;
                }
            } else {
                let mut end = *i + 1;
                while end < s.len() && !matches!(s[end], b' ' | b')' | b'$') {
                    end += 1;
                }
                last_expr = Expressions(vec![Item::from_str(&s[*i..end], map)]);
                *i = end;
            }
        }
        let mut result = last_expr;
        while !stack.is_empty() {
            result = result + stack.pop().unwrap();
        }
        result
    }
    let mut var_map = HashMap::new();
    for (var, val) in evalvars.iter().zip(&evalints) {
        var_map.insert(var.as_str(), *val);
    }
    let mut s = expression.into_bytes();
    s.push(b'$');
    let expr = dfs(&s, &mut 0, &var_map);
    expr.0.into_iter().filter_map(|x| x.to_string()).collect()
}

fn main() {
    assert_eq!(basic_calculator_iv(String::from("((a - b) * (b - c) + (c - a)) * ((a - b) + (b - c) * (c - a))"), vec![], vec![]), svec!["-1*a*a*b*b", "2*a*a*b*c", "-1*a*a*c*c", "1*a*b*b*b", "-1*a*b*b*c", "-1*a*b*c*c", "1*a*c*c*c", "-1*b*b*b*c", "2*b*b*c*c", "-1*b*c*c*c", "2*a*a*b", "-2*a*a*c", "-2*a*b*b", "2*a*c*c", "1*b*b*b", "-1*b*b*c", "1*b*c*c", "-1*c*c*c", "-1*a*a", "1*a*b", "1*a*c", "-1*b*c"]);
    assert_eq!(basic_calculator_iv(String::from("(e + 8) * (e - 8)"), vec![], vec![]), svec!["1*e*e", "-64"]);
    assert_eq!(basic_calculator_iv(String::from("e - 8 + temperature - pressure"), svec!["e", "temperature"], vec![1, 12]), svec!["-1*pressure", "5"]);
    assert_eq!(basic_calculator_iv(String::from("e + 8 - a + 5"), svec!["e"], vec![1]), svec!["-1*a", "14"]);
    assert_eq!(basic_calculator_iv(String::from("a * b * c + b * a * c * 4"), vec![], vec![]), svec!["5*a*b*c"]);
}
