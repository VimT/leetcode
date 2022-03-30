//! 链表随机节点


use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

use leetcode::link;
use leetcode::linknode::{ListNode, vec_to_link};

struct Solution {
    head: Option<Box<ListNode>>,
    rng: ThreadRng,
}


impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head, rng: thread_rng() }
    }

    // 蓄水池抽样算法
    fn get_random(&mut self) -> i32 {
        let mut count = 0;
        let mut reserve = 0;
        let mut cur = &self.head;
        while cur.is_some() {
            count += 1;
            let rand = self.rng.gen_range(1, count + 1);
            if rand == count {
                reserve = cur.as_ref().unwrap().val;
            }
            cur = &cur.as_ref().unwrap().next;
        }
        reserve
    }
}

fn main() {
    let mut s = Solution::new(link![1, 2, 3]);
    let mut cnt = vec![0; 4];
    for _ in 0..10000 {
        cnt[s.get_random() as usize] += 1;
    }
    for i in 1..=3 {
        println!("{}: {}", i, cnt[i]);
    }
}
