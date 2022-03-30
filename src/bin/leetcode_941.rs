//! 有效的山脉数组

pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let len = arr.len();
    if len < 3 { return false; }
    let mut i = 1;
    while i < len && arr[i] > arr[i - 1] {
        i += 1;
    }
    if i == len || i == 1 { return false; }
    while i < len && arr[i] < arr[i - 1] {
        i += 1;
    }
    i == len
}

fn main() {
    assert_eq!(valid_mountain_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), false);
    assert_eq!(valid_mountain_array(vec![2, 1]), false);
    assert_eq!(valid_mountain_array(vec![3, 5, 5]), false);
    assert_eq!(valid_mountain_array(vec![0, 3, 2, 1]), true);
}
