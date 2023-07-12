//! 重复至少 K 次且长度为 M 的模式

pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
    let len = arr.len();
    let m = m as usize;
    let k = k as usize;
    if len < m * k { return false; }
    for mut start in 0..len - m * k {
        let a = &arr[start..start + m];
        let mut ok = true;
        start += m;
        for _ in 1..k {
            if &arr[start..start + m] != a {
                ok = false;
                break;
            }
            start += m;
        }
        if ok {
            return true;
        }
    }
    false
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, m: i32, k: i32) -> bool) {
        assert_eq!(func(vec![1, 2, 4, 4, 4, 4], 1, 3), true);
        assert_eq!(func(vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2), true);
        assert_eq!(func(vec![1, 2, 1, 2, 1, 3], 2, 3), false);
        assert_eq!(func(vec![1, 2, 3, 1, 2], 2, 2), false);
        assert_eq!(func(vec![2, 2, 2, 2], 2, 3), false);
    }
    test(contains_pattern);
}
