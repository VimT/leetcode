//! 找不同

pub fn find_the_difference(s: String, t: String) -> char {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut sv = [0; 26];
    let mut tv = [0; 26];
    for &ch in s {
        sv[(ch - b'a') as usize] += 1;
    }
    for &ch in t {
        tv[(ch - b'a') as usize] += 1;
    }
    for i in 0..26 {
        if tv[i] - sv[i] == 1 {
            return (i as u8 + b'a') as char;
        }
    }
    panic!()
}

fn main() {
    assert_eq!(find_the_difference(String::from("abcd"), String::from("abcde")), 'e');
    assert_eq!(find_the_difference(String::from(""), String::from("y")), 'y');
    assert_eq!(find_the_difference(String::from("a"), String::from("aa")), 'a');
    assert_eq!(find_the_difference(String::from("ae"), String::from("aea")), 'a');
}
