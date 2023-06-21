//! 总行驶距离

pub fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
    let mut result = 0;
    while main_tank >= 5 {
        let t = main_tank / 5;
        result += t * 5;
        main_tank %= 5;
        let x = t.min(additional_tank);
        additional_tank -= x;
        main_tank += x;
    }
    (result + main_tank) * 10
}

fn main() {
    fn test(func: fn(main_tank: i32, additional_tank: i32) -> i32) {
        assert_eq!(func(9, 2), 110);
        assert_eq!(func(5, 10), 60);
        assert_eq!(func(1, 2), 10);
    }
    test(distance_traveled);
}
