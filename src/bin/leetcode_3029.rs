//! 将单词恢复初始状态所需的最短时间 I


pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
    let s = word.as_bytes();
    let len = s.len();
    let k = k as usize;
    let mx = (len + k - 1) / k;
    for x in 1..mx {
        if s[x * k..] == s[..len - x * k] {
            return x as i32;
        }
    }
    mx as i32
}

fn main() {
    fn test(func: fn(word: String, k: i32) -> i32) {
        assert_eq!(func(String::from("abacaba"), 3), 2);
        assert_eq!(func(String::from("abacaba"), 4), 1);
        assert_eq!(func(String::from("abcbabcd"), 2), 4);
    }
    test(minimum_time_to_initial_state);
}
