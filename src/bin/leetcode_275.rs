//! H 指数 II

pub fn h_index(citations: Vec<i32>) -> i32 {
    let len = citations.len();
    for i in 0..len {
        if (citations[i] as usize) >= len - i - 1 {
            return (len - i) as i32;
        }
    }
    0
}

pub fn h_index_log(citations: Vec<i32>) -> i32 {
    let len = citations.len() as i32;
    let mut left = 0;
    let mut right = len - 1;
    // another binary search
    while left <= right {
        let mid = left + (right - left) / 2;
        if citations[mid as usize] < (len - mid) {
            left = mid + 1;
        } else if citations[mid as usize] == (len - mid) {
            return len - mid;
        } else {
            right = mid - 1;
        }
    }
    len - left
}


fn main() {
    fn test(func: fn(citations: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0, 1, 3, 5, 6]), 3);
        assert_eq!(func(vec![1, 2, 100]), 2);
    }
    test(h_index);
    test(h_index_log);
}
