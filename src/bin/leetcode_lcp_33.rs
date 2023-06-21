//! 蓄水


use std::collections::BinaryHeap;

/// k为蓄水次数， 那么蓄水前，容量应该有 ceil(vat[i] / k)
pub fn store_water(bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
    let max_k = vat.iter().max().copied().unwrap();
    if max_k == 0 { return 0; }
    let mut result = i32::MAX;
    let mut k = 1;
    while k <= max_k.min(result) {
        let mut t = 0;
        for (&b, &v) in bucket.iter().zip(&vat) {
            t += 0.max((v + k - 1) / k - b); // 向上取整
        }
        result = result.min(t + k);
        k += 1;
    }
    result
}

/// 优先队列
pub fn store_water2(mut bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
    let len = bucket.len();
    let mut upgrade = 0;
    let mut heap = BinaryHeap::new();
    fn times(a: i32, b: i32) -> i32 {
        (a + b - 1) / b
    }
    for i in 0..len {
        if bucket[i] == 0 && vat[i] > 0 {
            upgrade += 1;
            bucket[i] += 1;
        }
        if vat[i] > 0 {
            heap.push((times(vat[i], bucket[i]), i));
        }
    }
    if heap.is_empty() {return 0;}
    let mut result = i32::MAX;
    while upgrade < result {
        let (v, i) = heap.pop().unwrap();
        result = result.min(upgrade + v);
        if v == 1 { break; }
        let t = times(vat[i], v - 1); // 少倒一次
        upgrade += t - bucket[i];
        bucket[i] = t;
        heap.push((times(vat[i], bucket[i]), i));
    }
    result
}

fn main() {
    fn test(func: fn(bucket: Vec<i32>, vat: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 2, 5], vec![0, 0, 0]), 0);
        assert_eq!(func(vec![1, 3], vec![6, 8]), 4);
        assert_eq!(func(vec![9, 0, 1], vec![0, 2, 2]), 3);
    }
    test(store_water);
    test(store_water2);
}
