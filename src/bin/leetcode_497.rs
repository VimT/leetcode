//! 非重叠矩形中的随机点


use std::collections::BTreeMap;

use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

struct Solution {
    sum: i32,
    rects: Vec<(i32, i32, i32, i32)>,
    areas: Vec<i32>,
    rng: ThreadRng,
}


impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut sum = 0;
        let mut new = Vec::with_capacity(rects.len());
        let mut areas = Vec::with_capacity(rects.len() + 1);
        for rect in rects {
            let area = (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
            sum += area;
            areas.push(sum);
            new.push((rect[0], rect[1], rect[2], rect[3]));
        }
        Self { sum, rects: new, areas, rng: thread_rng() }
    }

    fn pick(&mut self) -> Vec<i32> {
        let area = self.rng.gen_range(0, self.sum + 1);
        let idx = self.areas.binary_search(&area).unwrap_or_else(|x| x);
        let rect = self.rects[idx];
        let point = area - if idx > 0 { self.areas[idx - 1] } else { 0 };
        let width = rect.2 - rect.0 + 1;
        vec![rect.0 + point % width, rect.1 + point / width]
    }
}

fn main() {
    let mut s = Solution::new(vec![vec![1, 1, 5, 5]]);
    let mut m = BTreeMap::new();
    for _ in 0..1000 {
        *m.entry(s.pick()).or_insert(0i32) += 1;
    }
    for (k, v) in m {
        println!("{:?}: {}", k, v);
    }
}
