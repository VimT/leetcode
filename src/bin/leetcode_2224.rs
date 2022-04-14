//! 转化时间需要的最少操作数

pub fn convert_time(current: String, correct: String) -> i32 {
    let (hour, min) = current.split_once(':').unwrap();
    let cur = hour.parse::<i32>().unwrap() * 60 + min.parse::<i32>().unwrap();
    let (hour, min) = correct.split_once(':').unwrap();
    let target = hour.parse::<i32>().unwrap() * 60 + min.parse::<i32>().unwrap();
    let mut diff = target - cur;
    let mut result = 0;
    let divs = [60, 15, 5, 1];
    for div in divs {
        result += diff / div;
        diff %= div
    }
    result
}

fn main() {
    assert_eq!(convert_time(String::from("02:30"), String::from("04:35")), 3);
    assert_eq!(convert_time(String::from("11:00"), String::from("11:01")), 1);
}
