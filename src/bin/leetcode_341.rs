//! 扁平化嵌套列表迭代器

#[derive(Debug, PartialEq, Eq)]
enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}


struct NestedIterator {
    parsed: Vec<i32>,
    k: usize,
}

impl NestedIterator {
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

fn main() {
    let mut iter = NestedIterator::new(vec![NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]), NestedInteger::Int(2), NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)])]);
    let mut arr = vec![];
    while iter.has_next() {
        arr.push(iter.next());
    }
    assert_eq!(arr, [1, 1, 2, 1, 1]);
}