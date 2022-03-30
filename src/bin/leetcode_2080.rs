//! 区间内查询数字的频率

use std::collections::HashMap;

struct RangeFreqQuery {
    m: HashMap<i32, Vec<usize>>,
}


impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut m: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..arr.len() {
            m.entry(arr[i]).or_default().push(i);
        }
        RangeFreqQuery { m }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        if let Some(v) = self.m.get(&value) {
            let left = v.binary_search(&(left as usize)).map(|x| x as i32).unwrap_or_else(|x| x as i32);
            let right = v.binary_search(&(right as usize)).map(|x| x as i32).unwrap_or_else(|x| x as i32 - 1);
            return right - left + 1;
        }
        0
    }
}

fn main() {
    let rfq = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
    assert_eq!(rfq.query(1, 2, 4), 1);
    assert_eq!(rfq.query(0, 11, 33), 2);
}
