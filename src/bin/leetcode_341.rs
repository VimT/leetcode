//! 扁平化嵌套列表迭代器

#[derive(Debug, PartialEq, Eq)]
enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct Solution {
    parsed: Vec<i32>,
    k: usize,
}

impl Solution {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut v = vec![];
        Self::parse(&mut v, &nested_list);
        Self { parsed: v, k: 0 }
    }

    fn parse(v: &mut Vec<i32>, nested: &Vec<NestedInteger>) {
        for i in nested {
            match i {
                NestedInteger::Int(val) => v.push(*val),
                NestedInteger::List(nested) => Self::parse(v, &nested)
            }
        }
    }

    pub fn next(&mut self) -> i32 {
        let result = self.parsed[self.k];
        self.k += 1;
        return result;
    }

    pub fn has_next(&self) -> bool {
        self.k < self.parsed.len()
    }
}

fn main() {}