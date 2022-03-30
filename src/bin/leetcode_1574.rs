//! 删除最短的子数组使剩余数组有序

pub fn find_length_of_shortest_subarray(mut arr: Vec<i32>) -> i32 {
    arr.insert(0, i32::MIN);
    arr.push(i32::MAX);
    let len = arr.len();
    let mut left = 1;
    let mut right = len - 2;
    while left < len - 1 && arr[left] >= arr[left - 1] {
        left += 1;
    }
    if left == len - 1 { return 0; }
    let mut result = right + 1 - left;
    for left in (1..=left).rev() {
        while arr[right] >= arr[left - 1] && arr[right] <= arr[right + 1] {
            right -= 1;
        }
        result = result.min(right + 1 - left);
    }

    result as i32
}

fn main() {
    assert_eq!(find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]), 3);
    assert_eq!(find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]), 4);
    assert_eq!(find_length_of_shortest_subarray(vec![1, 2, 3]), 0);
}
