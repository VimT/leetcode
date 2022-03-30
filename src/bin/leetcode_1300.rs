//!  转变数组后最接近目标值的数组和

/// 区间二分查找
pub fn find_best_value(mut arr: Vec<i32>, target: i32) -> i32 {
    arr.sort_unstable();

    let len = arr.len();
    let mut prefix = vec![0; len + 1];
    for i in 1..=len {
        prefix[i] = prefix[i - 1] + arr[i - 1];
    }

    let cal_sum = |value: i32| {
        let mid = arr.binary_search(&value).unwrap_or_else(|x| x);
        prefix[mid] + value * (len - mid) as i32
    };
    let mut left = 0;
    let mut right = *arr.last().unwrap();
    while left < right {
        let mid = left + (right - left) / 2;
        let s = cal_sum(mid);
        if s < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    let small = (cal_sum(left) - target).abs();
    let big = (cal_sum(left - 1) - target).abs();
    return if small >= big { left - 1 } else { left };
}

fn main() {
    assert_eq!(find_best_value(vec![4, 9, 3], 10), 3);
    assert_eq!(find_best_value(vec![2, 3, 5], 10), 5);
    assert_eq!(find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803), 11361);
}

