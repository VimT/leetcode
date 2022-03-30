//! 超级丑数


use std::collections::BinaryHeap;

pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::with_capacity(n as usize);
    let mut ans = 1;
    for _ in 1..n {
        for &p in &primes {
            heap.push(-1 * p as i64 * ans as i64);
        }
        ans = -heap.pop().unwrap();
        while !heap.is_empty() && ans == -*heap.peek().unwrap() {
            heap.pop();
        }
    }
    ans as i32
}

pub fn nth_super_ugly_number_best(n: i32, primes: Vec<i32>) -> i32 {
    let mut res = Vec::with_capacity(n as usize);
    res.push(1);
    let k = primes.len();
    let mut idx = vec![0; k];
    let n = n as usize;

    for _ in 1..n {
        let mut ugly = i32::MAX;
        for i in 0..k {
            ugly = ugly.min(primes[i] * res[idx[i]]);
        }
        res.push(ugly);
        for i in 0..k {
            if ugly == primes[i] * res[idx[i]] {
                idx[i] += 1;
            }
        }
    }

    res[n - 1]
}

fn main() {
    fn test(func: fn(n: i32, primes: Vec<i32>) -> i32) {
        assert_eq!(func(12, vec![2, 7, 13, 19]), 32);
        assert_eq!(func(1, vec![2, 3, 5]), 1);
    }
    test(nth_super_ugly_number);
    test(nth_super_ugly_number_best);
}
