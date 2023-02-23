//! 最大平均通过率

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug, Ord)]
struct Class {
    pass: i64,
    total: i64,
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = (other.total + 1) * other.total * (self.total - self.pass);
        let b = (self.total + 1) * self.total * (other.total - other.pass);
        a.partial_cmp(&b)
    }
}


pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    let len = classes.len();
    let mut heap: BinaryHeap<Class> = classes.into_iter().map(|x| Class { pass: x[0] as i64, total: x[1] as i64 }).collect();
    for _ in 0..extra_students {
        let mut class = heap.pop().unwrap();
        class.pass += 1;
        class.total += 1;
        heap.push(class);
    }
    heap.into_iter().map(|x| x.pass as f64 / x.total as f64).sum::<f64>() / len as f64
}

fn main() {
    fn test(func: fn(classes: Vec<Vec<i32>>, extra_students: i32) -> f64) {
        fn verify(a: f64, b: f64) {
            assert!((a - b).abs() < 1e5, "{}", a)
        }
        verify(func(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2), 0.78333);
        verify(func(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4), 0.53485);
    }
    test(max_average_ratio);
}
