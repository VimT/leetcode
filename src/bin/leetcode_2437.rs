//! 有效时间的数目

pub fn count_time(time: String) -> i32 {
    let mut result = 0;
    let t = time.as_bytes();
    'out: for i in 0..1440 {
        let tt = format!("{:02}:{:02}", i / 60, i % 60);
        let s = tt.as_bytes();
        for i in 0..5 {
            if t[i] != b'?' && t[i] != s[i] {
                continue 'out;
            }
        }
        result += 1;
    }
    result
}


fn main() {
    assert_eq!(count_time(String::from("?5:00")), 2);
    assert_eq!(count_time(String::from("0?:0?")), 100);
    assert_eq!(count_time(String::from("??:??")), 1440);
}
