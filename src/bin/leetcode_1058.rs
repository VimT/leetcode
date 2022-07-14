//! 最小化舍入误差以满足目标

use leetcode::svec;

pub fn minimize_error(prices: Vec<String>, target: i32) -> String {
    let float_price: Vec<f64> = prices.iter().map(|x| x.parse::<f64>().unwrap()).collect();
    let min: i32 = float_price.iter().map(|x| *x as i32).sum();
    let max = min + prices.iter().filter(|x| !x.ends_with(".000")).count() as i32;
    if target < min || target > max {
        return String::from("-1");
    }
    let mut small: Vec<f64> = float_price.iter().map(|&x| x - x as i32 as f64).collect();
    let len = prices.len();
    small.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let mut result = 0.0;
    let mid = min + len as i32 - target;
    for i in 0..mid {
        result += small[i as usize];
    }
    for i in mid..len as i32 {
        result += 1. - small[i as usize];
    }
    format!("{:.3}", result)
}

fn main() {
    fn test(func: fn(prices: Vec<String>, target: i32) -> String) {
        assert_eq!(func(svec!["2.000","2.000","2.000","2.000","2.000"], 10), String::from("0.000"));
        assert_eq!(func(svec!["2.000","2.000","2.000","2.000","2.000"], 11), String::from("-1"));
        assert_eq!(func(svec!["0.700","2.800","4.900"], 8), String::from("1.000"));
        assert_eq!(func(svec!["1.500","2.500","3.500"], 10), String::from("-1"));
        assert_eq!(func(svec!["1.500","2.500","3.500"], 9), String::from("1.500"));
    }
    test(minimize_error);
}
