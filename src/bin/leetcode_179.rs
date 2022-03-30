//! 最大数

use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Value {
    num: String,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return [self.num.clone(), other.num.clone()].concat().partial_cmp(&[other.num.clone(), self.num.clone()].concat());
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        return [self.num.clone(), other.num.clone()].concat().cmp(&[other.num.clone(), self.num.clone()].concat());
    }
}

pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums: Vec<Value> = nums.into_iter().map(|x| Value { num: x.to_string() }).collect();
    nums.sort();
    nums.reverse();
    let ans: String = nums.into_iter().map(|x| x.num).collect();
    if &ans[0..1] == "0" {
        return "0".to_string();
    }
    return ans;
}

fn main() {
    assert_eq!(largest_number(vec![10, 2]), String::from("210"));
    assert_eq!(largest_number(vec![3, 30, 34, 5, 9]), String::from("9534330"));
}
