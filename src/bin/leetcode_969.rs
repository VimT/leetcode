//! 煎饼排序

/// 翻到第一位再翻回去
pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut n = arr.len() as i32;
    while n > 0 {
        let idx = arr.iter().position(|x| *x == n).unwrap();
        if idx as i32 + 1 != n {
            arr[..=idx].reverse();
            result.push(idx as i32 + 1);
            arr[..n as usize].reverse();
            result.push(n);
        }
        n -= 1;
    }
    result
}

fn main() {
    assert_eq!(pancake_sort(vec![3, 2, 4, 1]), vec![3, 4, 2, 3, 1, 2]);
    assert_eq!(pancake_sort(vec![1, 2, 3]), vec![]);
}
