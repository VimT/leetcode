use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut ans = KthLargest {
            heap: BinaryHeap::with_capacity(k as usize + 10),
            k: k as usize,
        };
        for i in nums {
            ans.add(i);
        }
        ans
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

fn main() {
    let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
    println!("{}", kth_largest.add(3));   // return 4
    println!("{}", kth_largest.add(5));   // return 5
    println!("{}", kth_largest.add(10));  // return 5
    println!("{}", kth_largest.add(9));   // return 8
    println!("{}", kth_largest.add(4));   // return 8
}
