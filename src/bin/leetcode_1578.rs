//! 使绳子变成彩色的最短时间

pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let s = colors.as_bytes();
    let len = s.len();
    let mut i = 0;
    let mut result = 0;
    while i < len {
        let c = s[i];
        let mut j = i + 1;
        let mut time_sum = needed_time[i];
        let mut time_max = needed_time[i];
        while j < len && s[j] == c {
            time_sum += needed_time[j];
            time_max = time_max.max(needed_time[j]);
            j += 1;
        }
        result += time_sum - time_max;
        i = j;
    }
    result
}

fn main() {
    fn test(func: fn(colors: String, needed_time: Vec<i32>) -> i32) {
        assert_eq!(func(String::from("abaac"), vec![1, 2, 3, 4, 5]), 3);
        assert_eq!(func(String::from("abc"), vec![1, 2, 3]), 0);
        assert_eq!(func(String::from("aabaa"), vec![1, 2, 3, 4, 1]), 2);
    }
    test(min_cost);
}
