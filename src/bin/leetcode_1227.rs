//! 飞机座位分配概率

pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
    if n <= 2 { return 1. / n as f64; }
    let mut prob = 0.5;
    for i in 3..=n {
        prob = (1. + ((i - 2) as f64 * prob)) / i as f64;
    }
    prob
}

fn main() {
    fn test(func: fn(n: i32) -> f64) {
        assert_eq!(func(1), 1.00000);
        assert_eq!(func(2), 0.50000);
    }
    test(nth_person_gets_nth_seat);
}
