//! 交换一次的先前排列

/// 贪心
pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    for i in (0..len - 1).rev() {
        if arr[i] > arr[i + 1] {
            let mut j = i + 1;
            while j < len && arr[j] < arr[i] {
                j += 1;
            }
            let mut k = j - 1;
            while arr[k] == arr[k - 1] {
                k -= 1;
            }
            arr.swap(i, k);
            break;
        }
    }
    arr
}

fn main() {
    assert_eq!(prev_perm_opt1(vec![3, 1, 1, 3]), vec![1, 3, 1, 3]);
    assert_eq!(prev_perm_opt1(vec![3, 2, 1]), vec![3, 1, 2]);
    assert_eq!(prev_perm_opt1(vec![1, 1, 5]), vec![1, 1, 5]);
    assert_eq!(prev_perm_opt1(vec![1, 9, 4, 6, 7]), vec![1, 7, 4, 6, 9]);
}
