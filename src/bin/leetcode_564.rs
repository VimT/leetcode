//! 寻找最近的回文数

pub fn nearest_palindromic(n: String) -> String {
    let nb = n.as_bytes();
    let len = nb.len();
    let num = n.parse::<i64>().unwrap();
    if num < 10 || num == 10i64.pow((len - 1) as u32) {
        return (num - 1).to_string();
    }
    if num - 1 == 10i64.pow((len - 1) as u32) {
        return (num - 2).to_string();
    }
    if num + 1 == 10i64.pow(len as u32) {
        return (num + 2).to_string();
    }
    let pre: i64 = n[..(len + 1) / 2].parse().unwrap();
    let mut tmp: Vec<i64> = vec![];
    for dx in [-1, 0, 1] {
        let mut s = (pre + dx).to_string().into_bytes();
        let mut revers_s = s.clone();
        revers_s.reverse();
        unsafe { s.set_len(len / 2); }
        s.extend_from_slice(&revers_s);
        unsafe { tmp.push(String::from_utf8_unchecked(s).parse().unwrap()) }
    }
    tmp.sort_unstable_by_key(|x| (*x - num).abs());
    tmp[if tmp[0] == num { 1 } else { 0 }].to_string()
}

fn main() {
    assert_eq!(nearest_palindromic("123".to_string()), "121");
}
