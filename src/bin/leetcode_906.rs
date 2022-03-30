//! 超级回文数

use std::sync::Once;

static mut MAP: Option<Vec<i64>> = None;
static mut ONCE: Once = Once::new();

fn isp(num: i64) -> bool {
    let s = num.to_string().into_bytes();
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        if s[l] != s[r] { return false; }
        l += 1;
        r -= 1;
    }
    true
}

fn make(cur: &mut Vec<u8>, len: usize, result: &mut Vec<i64>) {
    if cur.len() > (len - 1) / 2 {
        let mut num = cur.clone();
        for i in (0..len / 2).rev() {
            num.push(num[i]);
        }
        assert_eq!(num.len(), len);
        let mut rn = 0;
        for &ch in &num {
            rn = rn * 10 + ch as i64;
        }
        let double = rn * rn;
        if isp(double) { result.push(double); }
        return;
    }
    for i in if cur.is_empty() { 1 } else { 0 }..=2 {
        cur.push(i);
        make(cur, len, result);
        cur.pop();
    }
}

pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
    unsafe {
        ONCE.call_once(|| {
            let mut result = Vec::with_capacity(1000);
            result.push(1);
            result.push(4);
            result.push(9);
            for len in 2..10 {
                make(&mut vec![], len, &mut result);
            }
            println!("{:?}", result);
            MAP = Some(result);
        });

        if let Some(m) = &MAP {
            let left = left.parse::<i64>().unwrap();
            let right = right.parse::<i64>().unwrap();
            let l = m.binary_search(&left).unwrap_or_else(|x| x);
            let r = m.binary_search(&right).map(|x| x + 1).unwrap_or_else(|x| x);
            return (r - l) as i32;
        }
        panic!()
    }
}

fn main() {
    assert_eq!(superpalindromes_in_range(String::from("4"), String::from("1000")), 4);
    assert_eq!(superpalindromes_in_range(String::from("1"), String::from("2")), 1);
}
