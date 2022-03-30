//! 分割数组为连续子序列

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn is_possible(nums: Vec<i32>) -> bool {
    let mut m: HashMap<i32, BinaryHeap<Reverse<i32>>> = HashMap::new();
    for num in nums {
        if m.contains_key(&(num - 1)) {
            let Reverse(cnt) = m.get_mut(&(num - 1)).unwrap().pop().unwrap();
            if m.get(&(num - 1)).unwrap().len() == 0 { m.remove(&(num - 1)); }
            m.entry(num).or_default().push(Reverse(cnt + 1));
        } else {
            m.entry(num).or_default().push(Reverse(1));
        }
    }
    for (_, heap) in m {
        if heap.peek().unwrap().0 < 3 {
            return false;
        }
    }
    true
}

/// O(n)
pub fn is_possible_greedy(nums: Vec<i32>) -> bool {
    let mut count: HashMap<i32, i32> = HashMap::new();
    let mut end = HashMap::new();
    for &num in &nums {
        *count.entry(num).or_default() += 1;
    }
    for num in nums {
        let cnt = *count.get(&num).unwrap();
        if cnt > 0 {
            if *end.get(&(num - 1)).unwrap_or(&0) > 0 {
                *count.get_mut(&num).unwrap() -= 1;
                *end.entry(num).or_default() += 1;
                *end.entry(num - 1).or_default() -= 1;
            } else {
                if *count.get(&(num + 1)).unwrap_or(&0) > 0 && *count.get(&(num + 2)).unwrap_or(&0) > 0 {
                    *count.get_mut(&num).unwrap() -= 1;
                    *count.get_mut(&(num + 1)).unwrap() -= 1;
                    *count.get_mut(&(num + 2)).unwrap() -= 1;
                    *end.entry(num + 2).or_default() += 1;
                } else {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    assert_eq!(is_possible_greedy(vec![1, 2, 3, 3, 4, 5]), true);
    assert_eq!(is_possible_greedy(vec![1, 2, 3, 3, 4, 4, 5, 5]), true);
    assert_eq!(is_possible_greedy(vec![1, 2, 3, 4, 4, 5]), false);
}
