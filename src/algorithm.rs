use std::collections::BinaryHeap;

pub fn partition<T: Ord + Copy>(elements: &mut Vec<T>, start: usize, end: usize) -> usize {
    let mid = elements[end];
    let mut left = start;
    for i in start..end {
        if elements[i] < mid {
            elements.swap(left, i);
            left += 1;
        }
    }
    elements.swap(left, end);
    return left;
}

pub fn three_way_partition<T: Ord>(elements: &mut Vec<T>, target: T) {
    let mut next_big_pos = elements.len() - 1;
    let mut next_small_pos = 0;
    let mut next_scan_pos = 0;

    while next_scan_pos <= next_big_pos {
        if elements[next_scan_pos] > target {
            elements.swap(next_scan_pos, next_big_pos);
            next_big_pos -= 1;
        } else if elements[next_scan_pos] < target {
            elements.swap(next_scan_pos, next_small_pos);
            next_small_pos += 1;
            next_scan_pos += 1;
        } else {
            next_scan_pos += 1;
        }
    }
}

pub fn partition_reverse<T: Ord + Copy>(elements: &mut Vec<T>, start: usize, end: usize) -> usize {
    let mid = elements[end];
    let mut left = start;
    for i in start..end {
        if elements[i] > mid {
            elements.swap(i, left);
            left += 1;
        }
    }
    elements.swap(left, end);
    return left;
}


pub fn binary_search<T: Ord + Copy>(elements: &mut Vec<T>, start: usize, end: usize, key: T) -> Option<usize> {
    if start > end { return None; }
    let mid = start + (end - start) / 2;
    if elements[mid] > key {
        binary_search(elements, start, mid - 1, key)
    } else if elements[mid] < key {
        binary_search(elements, mid + 1, end, key)
    } else {
        Some(mid)
    }
}


pub fn quick_sort<T: Ord + Copy>(elements: &mut Vec<T>, start: usize, end: usize) {
    if start < end {
        let i = partition(elements, start, end);
        //这蛋疼的usize
        if i != 0 {
            quick_sort(elements, start, i - 1);
        }
        quick_sort(elements, i + 1, end);
    }
}


pub fn min_heapify<T: Ord>(nums: &mut Vec<T>, mut start: usize, end: usize) {
    if nums.len() < 1 || start > end { return; }
    let mut son = start * 2 + 1;
    while son <= end {
        if son + 1 <= end && nums[son + 1] < nums[son] { son += 1; }
        if nums[start] >= nums[son] {
            nums.swap(start, son);
            start = son;
            son = 2 * start + 1;
        } else {
            return;
        }
    }
}

pub fn max_heapify<T: Ord>(nums: &mut Vec<T>, mut start: usize, end: usize) {
    if nums.len() < 1 || start > end { return; }
    let mut son = start * 2 + 1;
    while son <= end {
        if son + 1 <= end && nums[son + 1] > nums[son] { son += 1; }
        if nums[start] <= nums[son] {
            nums.swap(start, son);
            start = son;
            son = 2 * start + 1;
        } else {
            return;
        }
    }
}

pub fn heap_sort<T: Ord>(nums: &mut Vec<T>) {
    // build
    let len = nums.len();
    for i in (0..len / 2).rev() {
        max_heapify(nums, i, len - 1);
    }
    // sort
    for i in (1..len).rev() {
        nums.swap(0, i);
        max_heapify(nums, 0, i - 1);
    }
}

pub fn nth_element<T: Ord + Copy>(nums: &mut Vec<T>, k: usize) {
    if nums.is_empty() { return; }
    let mut start = 0;
    let mut end = nums.len() - 1;
    while start < end {
        let m = partition(nums, start, end);
        if m == k {
            return;
        } else if m > k - 1 {
            end = m - 1;
        } else {
            start = m + 1;
        };
    }
}

pub fn partial_sort<T: Ord + Copy>(nums: &mut Vec<T>, k: usize) -> Vec<T> {
    if k <= 0 {
        return nums.clone();
    }
    let mut heap = BinaryHeap::with_capacity(k + 1);
    let mut ans = vec![nums[0]; nums.len()];

    for i in 0..k {
        heap.push(nums[i])
    }
    for i in k..nums.len() {
        if nums[i] < *heap.peek().unwrap() {
            heap.push(nums[i]);
            ans[i] = heap.pop().unwrap();
        }
    }
    for i in (0..k).rev() {
        ans[i] = heap.pop().unwrap();
    }
    ans
}


/// 树状数组
#[derive(Debug)]
pub struct BinIndexedTree {
    vec: Vec<i32>,
}

impl BinIndexedTree {
    pub fn with_capacity(capacity: usize) -> BinIndexedTree {
        BinIndexedTree { vec: vec![0; capacity] }
    }

    /// lowbit 返回1[末尾0的个数]
    #[inline]
    fn lowbit(v: usize) -> usize {
        // (-(v as i32) & v as i32) as usize
        (v ^ (v - 1)) & v
    }

    /// 将a[i]的值加上 delta，要将包含a[i]的值都加上delta
    pub fn add(&mut self, mut i: usize, delta: i32) {
        while i < self.vec.len() {
            self.vec[i] += delta;
            i += Self::lowbit(i);
        }
    }

    /// 求前缀和
    pub fn sum(&self, mut k: usize) -> i32 {
        let mut ans = 0;
        while k > 0 {
            ans += self.vec[k];
            k -= Self::lowbit(k);
        }
        ans
    }
}

impl From<Vec<i32>> for BinIndexedTree {
    fn from(v: Vec<i32>) -> Self {
        let mut ret = BinIndexedTree::with_capacity(v.len() + 1);
        for i in 1..=v.len() {
            ret.vec[i] = v[i - 1];
            for j in (i - Self::lowbit(i)..i - 1).rev() {
                ret.vec[i] += v[j];
            }
        }
        ret
    }
}

/// 部分匹配表：next[i] 表示 s[..i] 最长公共前后缀
pub fn max_match_length(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut next = vec![0; n];
    let mut j = 0;
    for (i, &ch) in s.iter().enumerate().skip(1) {
        while j > 0 && s[j] != ch { j = next[j - 1]; }
        if ch == s[j] { j += 1; }
        next[i] = j;
    }
    next
}

pub fn kmp(s: String, pattern: String) -> i32 {
    let query = s.as_bytes();
    let pattern = pattern.as_bytes();
    let n = query.len();
    let m = pattern.len();
    if m == 0 { return 0; }
    let next = max_match_length(pattern);
    let mut j = 0;
    for i in 0..n {
        while j > 0 && query[i] != pattern[j] {
            j = next[j - 1];
        }
        if query[i] == pattern[j] { j += 1; }
        if j == m {
            return (i + 1 - m) as i32;
        }
    }
    -1
}

pub fn asserting_cmp<T: PartialOrd>(a: &T, b: &T) -> std::cmp::Ordering {
    a.partial_cmp(b).expect("Comparing incomparable elements")
}

pub fn binary_search_lower<T: PartialOrd>(slice: &[T], key: &T) -> usize {
    slice.binary_search_by(|x| asserting_cmp(x, key).then(std::cmp::Ordering::Greater)).unwrap_err()
}

pub fn binary_search_upper<T: PartialOrd>(slice: &[T], key: &T) -> usize {
    slice.binary_search_by(|x| asserting_cmp(x, key).then(std::cmp::Ordering::Less)).unwrap_err()
}


pub fn quick_pow(mut base: i64, mut pow: i64, mod0: i64) -> i64 {
    base = base % mod0;
    let mut ans = 1;
    while pow != 0 {
        if pow & 1 == 1 {
            ans = ans * base % mod0;
        }
        base = base * base % mod0;
        pow >>= 1;
    }
    ans
}

pub fn matrix_mul(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>, mod_: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let m = b[0].len();
    let mut res = vec![vec![0; m]; n];
    for (i, row) in a.iter().enumerate() {
        for j in 0..m {
            for (k, v) in row.iter().enumerate() {
                res[i][j] = (res[i][j] + v * b[k][j]) % mod_;
            }
        }
    }
    res
}

pub fn matrix_power(mut base: Vec<Vec<i64>>, mut n: i64, mod_: i64) -> Vec<Vec<i64>> {
    let size = base.len();
    let mut res = vec![vec![0; size]; size];
    for i in 0..size { res[i][i] = 1; }
    while n > 0 {
        if n & 1 == 1 {
            res = matrix_mul(&res, &base, mod_);
        }
        base = matrix_mul(&base, &base, mod_);
        n >>= 1;
    }
    res
}

/// 求逆元
pub fn mod_inv(x: i64, m: i64) -> i64 {
    let (mut a, mut b, mut u, mut v) = (x, m, 1, 0);
    while b != 0 {
        let q = a / b;
        a -= b * q;
        u -= v * q;
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut u, &mut v);
    }
    ((u % m) + m) % m
}

pub fn cal_is_prime(n: usize) -> Vec<bool> {
    let mut result = vec![true; n + 1];
    result[1] = false;
    for i in 2..=n {
        if result[i] {
            let mut j = i * i;
            while j <= n {
                result[j] = false;
                j += i;
            }
        }
    }
    result
}

/// 欧拉筛
pub fn cal_prime(n: usize) -> Vec<usize> {
    let mut result = vec![];
    let mut is_prime = vec![true; n + 1];
    for i in 2..=n {
        if is_prime[i] { result.push(i); }
        for &p in &result {
            if i * p >= n { break; }
            is_prime[i * p] = false;
            if i % p == 0 { break; }
        }
    }
    result
}

/// 计算每个数的最小质因子
pub fn cal_lpf(n: usize) -> Vec<i32> {
    let mut lpf = vec![0; n + 1];
    for i in 2..=n {
        if lpf[i] == 0 {
            let mut j = i;
            while j <= n {
                if lpf[j] == 0 { lpf[j] = i as i32 };
                j += i;
            }
        }
    }
    lpf
}

/// 计算每个数有多少个不同的质因子
pub fn cal_prime_cnt(n: usize) -> Vec<i32> {
    let mut result = vec![0; n];
    for i in 2..n {
        if result[i] == 0 {
            let mut j = i;
            while j < n {
                result[j] += 1;
                j += i;
            }
        }
    }
    result
}

/// 逆元求组合数
pub fn combination_num(len: usize, mod_: i64) -> fn(i64, i64) -> i64 {
    unsafe {
        static mut FAC: Vec<i64> = vec![];
        static mut FACINV: Vec<i64> = vec![];
        static mut MOD: i64 = 0;
        FAC = vec![1; len];
        FACINV = vec![1; len];
        MOD = mod_;
        unsafe fn grow(before: usize, new: usize) {
            FAC.resize(new + 1, 0);
            for i in before..=new {
                FAC[i] = FAC[i - 1] * i as i64 % MOD;
            }
            FACINV.resize(new + 1, 0);
            FACINV[new] = quick_pow(FAC[new], MOD - 2, MOD);
            for i in (before + 1..=new).rev() {
                FACINV[i - 1] = FACINV[i] * i as i64 % MOD;
            }
        }
        grow(1, len - 1);
        |n: i64, m: i64| -> i64 {
            if m < 0 || n < m { return 0; }
            if n as usize >= FAC.len() { grow(FAC.len(), n as usize); }
            FAC[n as usize] * FACINV[m as usize] % MOD * FACINV[(n - m) as usize] % MOD
        }
    }
}

#[test]
fn test_quick_sort() {
    let mut elements = vec![2, 1, 5, 4, 7, 9, 5, 2, 1, 0];
    let ele = &mut elements;
    quick_sort(ele, 0, ele.len() - 1);
    println!("{:?}", ele);

    let mut elements = vec![1, 1, 1, 1, 1, 1];
    quick_sort(&mut elements, 0, 5);
    println!("{:?}", elements);
}


#[test]
fn test_heap_sort() {
    let mut elements = vec![2, 1, 5, 4, 7, 9, 5, 2, 1, 0];
    let ele = &mut elements;
    heap_sort(ele);
    println!("{:?}", ele);
}

#[test]
fn test_nth_element() {
    let mut elements = vec![2, 1, 5, 4, 7, 9, 5, 2, 1, 0];
    nth_element(&mut elements, 5);
    println!("{:?}", elements)
}

#[test]
fn test_partial_sort() {
    let mut elements = vec![2, 1, 5, 4, 7, 9, 5, 3, 1, 0];
    println!("{:?}", partial_sort(&mut elements, 5))
}

#[test]
fn test_bin_indexed_tree() {
    let elements = vec![4, 5, 6, 7];
    let tree = BinIndexedTree::from(elements);
    println!("{}", tree.sum(3));
}