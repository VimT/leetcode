//! 交换后字典序最小的字符串

pub fn get_smallest_string(mut s: String) -> String {
    let ss = unsafe { s.as_bytes_mut() };
    let len = ss.len();
    for i in 0..len - 1 {
        if ((ss[i] - b'0') & 1 == (ss[i + 1] - b'0') & 1) && ss[i] > ss[i + 1] {
            ss.swap(i, i + 1);
            break;
        }
    }
    s
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("45320")), String::from("43520"));
        assert_eq!(func(String::from("001")), String::from("001"));
    }
    test(get_smallest_string);
}
