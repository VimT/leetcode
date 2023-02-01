//! 最接近的因数

pub fn closest_divisors(num: i32) -> Vec<i32> {
    let mut result = vec![1, num + 1];
    let mut diff = num;
    for num in [num + 1, num + 2] {
        for a in (2..=(num as f64).sqrt() as i32).rev() {
            if num % a == 0 {
                let b = num / a;
                if b - a < diff {
                    result = vec![a, b];
                    diff = b - a;
                }
            }
        }
    }
    result
}

fn main() {
    use leetcode::unorder;
    fn test(func: fn(num: i32) -> Vec<i32>) {
        assert_eq!(unorder(func(8)), unorder(vec![3, 3]));
        assert_eq!(unorder(func(123)), unorder(vec![5, 25]));
        assert_eq!(unorder(func(999)), unorder(vec![40, 25]));
    }
    test(closest_divisors);
}
