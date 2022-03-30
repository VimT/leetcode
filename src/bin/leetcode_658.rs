//! 找到 K 个最接近的元素

pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let len = arr.len();
    let mut target = arr.binary_search(&x).unwrap_or_else(|x| x);
    if target > 0 && target < len && x - arr[target - 1] <= arr[target] - x { target -= 1; }
    if target == len { target -= 1; }
    let mut left = target;
    let mut right = target;
    while (right + 1 - left) < k as usize {
        let left_dis = if left > 0 { x - arr[left - 1] } else { i32::MAX };
        let right_dis = if right + 1 < len { arr[right + 1] - x } else { i32::MAX };
        if left_dis <= right_dis {
            left -= 1;
        } else {
            right += 1;
        }
    }
    arr[left..=right].to_vec()
}

fn main() {
    assert_eq!(find_closest_elements(vec![1, 3], 1, 2), vec![1]);
    assert_eq!(find_closest_elements(vec![3, 5, 8, 10], 2, 15), vec![8, 10]);
    assert_eq!(find_closest_elements(vec![0, 0, 1, 2, 3, 3, 4, 7, 7, 8], 3, 5), vec![3, 3, 4]);
    assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3), vec![1, 2, 3, 4]);
    assert_eq!(find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1), vec![1, 2, 3, 4]);
}
