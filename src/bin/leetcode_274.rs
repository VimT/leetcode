//! H 指数

pub fn h_index_sort(mut citations: Vec<i32>) -> i32 {
    citations.sort_unstable();
    let mut i = 0;
    let len = citations.len();
    while i < len && citations[len - 1 - i] > i as i32 {
        i += 1;
    }
    i as i32
}

pub fn h_index(citations: Vec<i32>) -> i32 {
    let len = citations.len();
    let mut papers = vec![0; len + 1];
    for i in citations {
        papers[len.min(i as usize)] += 1;
    }
    let mut k = len;
    let mut s = papers[len];
    while k > s {
        k -= 1;
        s += papers[k];
    }
    k as i32
}

fn main() {
    fn test(func: fn(citations: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 0, 6, 1, 5]), 3);
        assert_eq!(func(vec![1, 3, 1]), 1);
    }
    test(h_index);
    test(h_index_sort);
}
