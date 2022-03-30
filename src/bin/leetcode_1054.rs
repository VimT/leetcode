//! 距离相等的条形码


use std::collections::{BinaryHeap, HashMap};

pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    let len = barcodes.len();
    let mut cnt = HashMap::new();
    for barcode in barcodes {
        *cnt.entry(barcode).or_insert(0i32) += 1;
    }
    let mut heap = BinaryHeap::new();
    for (k, v) in cnt {
        heap.push((v, k));
    }
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        let (mut cnt, mut k) = heap.pop().unwrap();
        if !result.is_empty() && *result.last().unwrap() == k {
            let (cnt2, k2) = heap.pop().unwrap();
            heap.push((cnt, k));
            cnt = cnt2;
            k = k2;
        }
        result.push(k);
        if cnt > 1 {
            heap.push((cnt - 1, k));
        }
    }
    result
}

/// 隔位插入
pub fn rearrange_barcodes2(barcodes: Vec<i32>) -> Vec<i32> {
    let len = barcodes.len();
    let mut cnt = HashMap::new();
    for barcode in barcodes {
        *cnt.entry(barcode).or_insert(0i32) += 1;
    }
    let mut cnt: Vec<(i32, i32)> = cnt.into_iter().collect();
    cnt.sort_unstable_by_key(|x| -x.1);
    let mut help = Vec::with_capacity(len);
    for (k, num) in cnt {
        for _ in 0..num {
            help.push(k);
        }
    }
    let mut result = vec![0; len];
    let mut p = 0;
    for i in (0..len).step_by(2) {
        result[i] = help[p];
        p += 1;
    }
    for i in (1..len).step_by(2) {
        result[i] = help[p];
        p += 1;
    }
    result
}

fn main() {
    fn verify(list: Vec<i32>) {
        for win in list.windows(2) {
            assert_ne!(win[0], win[1]);
        }
    }
    fn test(func: fn(barcodes: Vec<i32>) -> Vec<i32>) {
        verify(func(vec![1, 1, 1, 2, 2, 2]));
        verify(func(vec![1, 1, 1, 1, 2, 2, 3, 3]));
    }
    test(rearrange_barcodes);
    test(rearrange_barcodes2);
}
