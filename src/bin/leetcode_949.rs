//! 给定数字能组成的最大时间

pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
    let mut result = -1;
    for i in 0..4 {
        for j in 0..4 {
            if i == j { continue; }
            for k in 0..4 {
                if k == i || k == j { continue; }
                let l = 6 - i - j - k;
                let hour = arr[i] * 10 + arr[j];
                let minute = arr[k] * 10 + arr[l];
                if hour < 24 && minute < 60 {
                    result = result.max(hour * 60 + minute);
                }
            }
        }
    }
    if result >= 0 {
        format!("{:02}:{:02}", result / 60, result % 60)
    } else {
        String::new()
    }
}

fn main() {
    assert_eq!(largest_time_from_digits(vec![0, 0, 0, 1]), String::from("10:00"));
    assert_eq!(largest_time_from_digits(vec![0, 0, 0, 0]), String::from("00:00"));
    assert_eq!(largest_time_from_digits(vec![1, 2, 3, 4]), String::from("23:41"));
    assert_eq!(largest_time_from_digits(vec![5, 5, 5, 5]), String::from(""));
}
