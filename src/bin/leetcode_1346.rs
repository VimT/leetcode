//! 检查整数及其两倍数是否存在

pub fn check_if_exist(mut arr: Vec<i32>) -> bool {
    arr.sort_unstable();
    let mut zero_cnt = 0;
    for &num in &arr {
        if num == 0 {
            if zero_cnt == 1 { return true; } else { zero_cnt += 1; }
        } else {
            if arr.binary_search(&(num * 2)).is_ok() { return true; }
        }
    }
    false
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> bool) {
        assert_eq!(func(vec![0, 1]), false);
        assert_eq!(func(vec![0, 0]), true);
        assert_eq!(func(vec![10, 2, 5, 3]), true);
        assert_eq!(func(vec![3, 1, 7, 11]), false);
    }
    test(check_if_exist);
}
