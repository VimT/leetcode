//! 最多能完成排序的块

pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut sorted = arr.clone();
    sorted.sort_unstable();
    let mut result = 0;
    let mut cur_max = 0;
    for (x, y) in arr.into_iter().zip(sorted) {
        cur_max = cur_max.max(x);
        if cur_max == y {
            result += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    assert_eq!(max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
}
