//! 并行课程 III

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut outv = vec![vec![]; n + 1];
    let mut ind = vec![0; n + 1];
    for i in relations {
        outv[i[0] as usize].push(i[1] as usize);
        ind[i[1] as usize] += 1;
    }
    let mut s = BinaryHeap::new();
    let mut result = 0;
    for i in 1..=n {
        if ind[i] == 0 {
            s.push(Reverse((time[i - 1], i)));
        }
    }
    while !s.is_empty() {
        let Reverse((cur_time, c)) = s.pop().unwrap();
        result = result.max(cur_time);
        for &j in &outv[c] {
            ind[j] -= 1;
            if ind[j] == 0 {
                s.push(Reverse((cur_time + time[j - 1], j)));
            }
        }
    }
    result
}

fn main() {
    assert_eq!(minimum_time(7,
                            vec![vec![1, 7], vec![1, 4], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 3], vec![7, 3], vec![7, 6]],
                            vec![6, 5, 1, 8, 2, 9, 4]), 19);
    assert_eq!(minimum_time(9, vec![vec![2, 7], vec![2, 6], vec![3, 6], vec![4, 6], vec![7, 6], vec![2, 1], vec![3, 1], vec![4, 1], vec![6, 1], vec![7, 1], vec![3, 8], vec![5, 8], vec![7, 8], vec![1, 9], vec![2, 9], vec![6, 9], vec![7, 9]], vec![9, 5, 9, 5, 8, 7, 7, 8, 4]), 32);
    assert_eq!(minimum_time(3, vec![vec![1, 3], vec![2, 3]], vec![3, 2, 5]), 8);
    assert_eq!(minimum_time(5, vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]], vec![1, 2, 3, 4, 5]), 12);
}
