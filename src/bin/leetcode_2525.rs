//! 根据规则将箱子分类

pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    String::from({
        let v = length as i64 * width as i64 * height as i64;
        const A: i32 = 1e4 as i32;
        let bulky = (length >= A || width >= A || height >= A) || (v >= 1e9 as i64);
        let heavy = mass >= 100;
        match (bulky, heavy) {
            (true, true) => "Both",
            (false, false) => "Neither",
            (true, false) => "Bulky",
            (false, true) => "Heavy"
        }
    })
}

fn main() {
    fn test(func: fn(length: i32, width: i32, height: i32, mass: i32) -> String) {
        assert_eq!(func(2909, 3968, 3272, 727), String::from("Both"));
        assert_eq!(func(1000, 35, 700, 300), String::from("Heavy"));
        assert_eq!(func(200, 50, 800, 50), String::from("Neither"));
    }
    test(categorize_box);
}
