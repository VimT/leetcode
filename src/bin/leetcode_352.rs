//! 将数据流变为多个不相交区间

use std::collections::BTreeMap;

struct SummaryRanges {
    tree: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            tree: BTreeMap::new()
        }
    }

    fn add_num(&mut self, val: i32) {
        let prev = self.tree.range(..=val).last().clone();
        if let Some((&start, &end)) = prev {
            // in
            if val <= end {
                return;
            } else if val == end + 1 {
                // merge
                if let Some(&next_end) = self.tree.get(&(val + 1)) {
                    self.tree.remove(&(val + 1));
                    self.tree.insert(start, next_end);
                    return;
                } else {
                    // right
                    self.tree.insert(start, val);
                    return;
                }
            }
        }
        // left
        if let Some(&next_end) = self.tree.get(&(val + 1)) {
            self.tree.remove(&(val + 1));
            self.tree.insert(val, next_end);
            return;
        }
        self.tree.insert(val, val);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.tree.iter().map(|(k, v)| vec![*k, *v]).collect()
    }
}


fn main() {
    let mut sr = SummaryRanges::new();
    sr.add_num(1);      // arr = [1]
    assert_eq!(sr.get_intervals(), [[1, 1]]);
    sr.add_num(3);      // arr = [1, 3]
    assert_eq!(sr.get_intervals(), [[1, 1], [3, 3]]);
    sr.add_num(7);      // arr = [1, 3, 7]
    assert_eq!(sr.get_intervals(), [[1, 1], [3, 3], [7, 7]]);
    sr.add_num(2);      // arr = [1, 2, 3, 7]
    assert_eq!(sr.get_intervals(), [[1, 3], [7, 7]]);
    sr.add_num(6);      // arr = [1, 2, 3, 6, 7]
    assert_eq!(sr.get_intervals(), [[1, 3], [6, 7]]);
    sr.add_num(2);
    assert_eq!(sr.get_intervals(), [[1, 3], [6, 7]]);
    sr.add_num(10);
    assert_eq!(sr.get_intervals(), [[1, 3], [6, 7], [10, 10]]);
    sr.add_num(11);
    assert_eq!(sr.get_intervals(), [[1, 3], [6, 7], [10, 11]]);
    sr.add_num(9);
    assert_eq!(sr.get_intervals(), [[1, 3], [6, 7], [9, 11]]);
}
