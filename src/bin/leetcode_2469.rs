//! 温度转换

pub fn convert_temperature(celsius: f64) -> Vec<f64> {
    vec![celsius + 273.15, celsius * 1.80 + 32.00]
}

fn main() {
    fn test(func: fn(celsius: f64) -> Vec<f64>) {
        assert_eq!(func(36.50), vec![309.65000, 97.70000]);
        assert_eq!(func(122.11), vec![395.26000, 251.79800]);
    }
    test(convert_temperature);
}
