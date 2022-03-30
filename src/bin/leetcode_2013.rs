//! 检测正方形

use std::collections::HashMap;

struct DetectSquares {
    m: HashMap<(i32, i32), i32>,
}

impl DetectSquares {
    fn new() -> Self {
        Self { m: HashMap::new() }
    }

    fn add(&mut self, point: Vec<i32>) {
        *self.m.entry((point[0], point[1])).or_default() += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let (x1, y1) = (point[0], point[1]);
        let mut result = 0;
        for (&(x2, y2), cnt) in &self.m {
            if x1 != x2 && y1 != y2 && (x2 - x1).abs() == (y2 - y1).abs() {
                if let Some(&cnt2) = self.m.get(&(x1, y2)) {
                    if let Some(&cnt3) = self.m.get(&(x2, y1)) {
                        result += cnt * cnt2 * cnt3;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ds = DetectSquares::new();
    ds.add(vec![3, 10]);
    ds.add(vec![11, 2]);
    ds.add(vec![3, 2]);
    assert_eq!(ds.count(vec![11, 10]), 1); // 返回 1 。你可以选择：
    //   - 第一个，第二个，和第三个点
    assert_eq!(ds.count(vec![14, 8]), 0);  // 返回 0 。查询点无法与数据结构中的这些点构成正方形。
    ds.add(vec![11, 2]);    // 允许添加重复的点。
    assert_eq!(ds.count(vec![11, 10]), 2); // 返回 2 。你可以选择：
    //   - 第一个，第二个，和第三个点
    //   - 第一个，第三个，和第四个点
}