//! 不动点

pub fn fixed_point(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] < mid as i32 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if left < arr.len() && arr[left] == left as i32 { left as i32 } else { -1 }
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> i32) {
        assert_eq!(func(vec![-10]), -1);
        assert_eq!(func(vec![-10, -5, 0, 3, 7]), 3);
        assert_eq!(func(vec![0, 2, 5, 8, 17]), 0);
        assert_eq!(func(vec![-10, -5, 3, 4, 7, 9]), -1);
    }
    test(fixed_point);
}
