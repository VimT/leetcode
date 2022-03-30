//! 数组中的最长山脉

pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut result = 0;
    let mut cur = 1;
    let mut up_or_down = 0;
    for i in 1..len {
        if up_or_down <= 1 {
            cur += 1;
            if arr[i] == arr[i - 1] || (up_or_down == 0 && arr[i] < arr[i - 1]) {
                cur = 1;
                up_or_down = 0;
            } else if arr[i] > arr[i - 1] {
                up_or_down = 1;
            } else {
                up_or_down = 2;
                result = result.max(cur);
            }
        } else {
            if arr[i] > arr[i - 1] {
                up_or_down = 1;
                cur = 2;
            } else if arr[i] == arr[i - 1] {
                up_or_down = 0;
                cur = 1;
            } else {
                cur += 1;
                result = result.max(cur);
            }
        }
    }
    result
}

fn main() {
    assert_eq!(longest_mountain(vec![875, 884, 239, 731, 723, 685]), 4);
    assert_eq!(longest_mountain(vec![0, 1, 0]), 3);
    assert_eq!(longest_mountain(vec![5, 4, 3, 2, 1, 2, 3, 4, 5, 4, 3, 2, 1]), 9);
    assert_eq!(longest_mountain(vec![5, 4, 3, 2, 1, 2, 3, 4, 5]), 0);
    assert_eq!(longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
    assert_eq!(longest_mountain(vec![2, 2, 2]), 0);
}
