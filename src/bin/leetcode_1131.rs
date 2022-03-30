//! 绝对值表达式的最大值

pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    let mut d = vec![];
    for (i, (x, y)) in arr1.into_iter().zip(arr2).enumerate() {
        a.push(x + y + i as i32);
        b.push(x + y - i as i32);
        c.push(x - y + i as i32);
        d.push(x - y - i as i32);
    }
    let cal = |a: Vec<i32>| {
        a.iter().max().unwrap() - a.iter().min().unwrap()
    };
    *[cal(a), cal(b), cal(c), cal(d)].iter().max().unwrap()
}

fn main() {
    fn test(func: fn(arr1: Vec<i32>, arr2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4], vec![-1, 4, 5, 6]), 13);
        assert_eq!(func(vec![1, -2, -5, 0, 10], vec![0, -2, -1, -7, -4]), 20);
    }
    test(max_abs_val_expr);
}
