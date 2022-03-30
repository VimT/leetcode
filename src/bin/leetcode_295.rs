//! 数据流的中位数

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default, Debug)]
struct MedianFinder {
    //大顶堆，存小数字
    little: BinaryHeap<i32>,
    //小顶堆，存大数字
    big: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        self.little.push(num);
        self.big.push(Reverse(self.little.pop().unwrap()));

        if self.little.len() < self.big.len() {
            self.little.push(self.big.pop().unwrap().0)
        }
    }

    fn find_median(&self) -> f64 {
        return if self.little.len() > self.big.len() {
            *self.little.peek().unwrap() as f64
        } else {
            (*self.little.peek().unwrap() + self.big.peek().unwrap().0) as f64 / 2 as f64
        };
    }
}

fn main() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    assert_eq!(obj.find_median(), 1.5);
    obj.add_num(3);
    assert_eq!(obj.find_median(), 2.0);
}

