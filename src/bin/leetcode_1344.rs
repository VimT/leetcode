//! 时钟指针的夹角

pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let min = 6. * minutes as f64;
    let hour = 30. * (hour as f64 + minutes as f64 / 60.);
    let mut result = (min - hour).abs();
    while result >= 360. {
        result -= 360.
    }
    result.min(360. - result)
}

fn main() {
    fn test(func: fn(hour: i32, minutes: i32) -> f64) {
        assert_eq!(func(1, 57), 76.5);
        assert_eq!(func(12, 0), 0.);
        assert_eq!(func(12, 30), 165.);
        assert_eq!(func(3, 30), 75.);
        assert_eq!(func(3, 15), 7.5);
    }
    test(angle_clock);
}
