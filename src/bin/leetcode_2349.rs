//! 设计数字容器系统

use std::collections::{BinaryHeap, HashMap};


#[derive(Default)]
struct NumberContainers {
    num_index: HashMap<i32, BinaryHeap<i32>>,
    index_num: HashMap<i32, i32>,
}


impl NumberContainers {
    fn new() -> Self {
        Default::default()
    }

    fn change(&mut self, index: i32, number: i32) {
        self.index_num.insert(index, number);
        self.num_index.entry(number).or_default().push(-index);
    }

    fn find(&mut self, number: i32) -> i32 {
        if let Some(index_list) = self.num_index.get_mut(&number) {
            while !index_list.is_empty() && *self.index_num.get(&(-index_list.peek().unwrap())).unwrap() != number {
                index_list.pop();
            }
            -index_list.peek().cloned().unwrap_or(1)
        } else {
            -1
        }
    }
}

fn main() {
    let mut nc = NumberContainers::new();
    assert_eq!(nc.find(10), -1); // 没有数字 10 ，所以返回 -1 。
    nc.change(2, 10); // 容器中下标为 2 处填入数字 10 。
    nc.change(1, 10); // 容器中下标为 1 处填入数字 10 。
    nc.change(3, 10); // 容器中下标为 3 处填入数字 10 。
    nc.change(5, 10); // 容器中下标为 5 处填入数字 10 。
    assert_eq!(nc.find(10), 1); // 数字 10 所在的下标为 1 ，2 ，3 和 5 。因为最小下标为 1 ，所以返回 1 。
    nc.change(1, 20); // 容器中下标为 1 处填入数字 20 。注意，下标 1 处之前为 10 ，现在被替换为 20 。
    assert_eq!(nc.find(10), 2); // 数字 10 所在下标为 2 ，3 和 5 。最小下标为 2 ，所以返回 2 。
}
