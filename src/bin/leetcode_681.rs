//! 最近时刻

pub fn next_closest_time(time: String) -> String {
    let s = time.as_bytes();
    let mut nums = vec![];
    for i in [0, 1, 3, 4] {
        if !nums.contains(&(s[i] - b'0')) {
            nums.push(s[i] - b'0');
        }
    }
    let cur_time_int = ((s[0] - b'0') * 10 + (s[1] - b'0')) as i32 * 60 + ((s[3] - b'0') * 10 + (s[4] - b'0')) as i32;
    let mut min_diff = i32::MAX;
    let mut result = time;
    for &a in &nums {
        for &b in &nums {
            if a * 10 + b < 24 {
                for &c in &nums {
                    for &d in &nums {
                        if c * 10 + d < 60 {
                            let time_int = (a * 10 + b) as i32 * 60 + (c * 10 + d) as i32;
                            let diff = (time_int - cur_time_int + 1440) % 1440;
                            if diff != 0 && diff < min_diff {
                                min_diff = diff;
                                result = format!("{}{}:{}{}", a, b, c, d);
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(time: String) -> String) {
        assert_eq!(func(String::from("00:00")), String::from("00:00"));
        assert_eq!(func(String::from("19:34")), String::from("19:39"));
        assert_eq!(func(String::from("23:59")), String::from("22:22"));
    }
    test(next_closest_time);
}
