//! 从房屋收集雨水需要的最少水桶数

pub fn minimum_buckets(street: String) -> i32 {
    let mut result = 0;
    let s = street.as_bytes();
    let len = s.len();
    if len == 1 && s[0] == b'H' { return -1; }
    if len > 1 && ((s[0] == b'H' && s[1] == b'H') || (s[len - 1] == b'H' && s[len - 2] == b'H')) { return -1; }
    let mut ok = vec![false; len];
    for i in 0..len {
        if s[i] == b'H' {
            if i > 1 && s[i - 2] == b'H' {
                if s[i - 1] == b'H' {
                    return -1;
                }
                if !ok[i - 2] {
                    result += 1;
                    ok[i] = true;
                }
            } else {
                ok[i] = true;
                result += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(minimum_buckets(String::from(".HH.H.H.H..")), 3);
    assert_eq!(minimum_buckets(String::from("H..H")), 2);
    assert_eq!(minimum_buckets(String::from(".H.H.")), 1);
    assert_eq!(minimum_buckets(String::from(".HHH.")), -1);
    assert_eq!(minimum_buckets(String::from("H")), -1);
    assert_eq!(minimum_buckets(String::from(".")), 0);
}
