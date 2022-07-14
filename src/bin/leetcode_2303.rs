//! 计算应缴税款总额

pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
    let mut last = 0;
    let mut result = 0.0;
    for bracket in brackets {
        if income >= bracket[0] {
            result += (bracket[0] - last) as f64 * bracket[1] as f64 / 100.0;
        } else {
            result += (income - last) as f64 * bracket[1] as f64 / 100.0;
            break;
        }
        last = bracket[0];
    }
    result
}

fn main() {
    fn test(func: fn(brackets: Vec<Vec<i32>>, income: i32) -> f64) {
        assert_eq!(func(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10), 2.65000);
        assert_eq!(func(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2), 0.25000);
        assert_eq!(func(vec![vec![2, 50]], 0), 0.00000);
    }
    test(calculate_tax);
}
