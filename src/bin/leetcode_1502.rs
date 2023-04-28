//! 判断能否形成等差数列

pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort_unstable();
    let diff = arr[1] - arr[0];
    for w in arr.windows(2) {
        if w[1] - w[0] != diff { return false; }
    }
    true
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> bool) {
        assert_eq!(func(vec![3, 5, 1]), true);
        assert_eq!(func(vec![1, 2, 4]), false);
    }
    test(can_make_arithmetic_progression);
}
