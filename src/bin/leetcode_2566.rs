//! 替换一个数字后的最大差值

pub fn min_max_difference(num: i32) -> i32 {
    let mut max = num;
    let mut min = num;
    let s = num.to_string();
    for i in 0..=9 {
        for j in 0..=9 {
            if i != j {
                let num2: i32 = s.replace(&i.to_string(), &j.to_string()).parse().unwrap();
                max = max.max(num2);
                min = min.min(num2);
            }
        }
    }
    max - min
}

fn main() {
    fn test(func: fn(num: i32) -> i32) {
        assert_eq!(func(11891), 99009);
        assert_eq!(func(90), 99);
    }
    test(min_max_difference);
}
