//! 满足三条件之一需改变的最少字符数

pub fn min_characters(a: String, b: String) -> i32 {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let alen = a.len() as i32;
    let blen = b.len() as i32;
    let mut acount = vec![0; 26];
    let mut bcount = vec![0; 26];
    for i in a { acount[(*i - b'a') as usize] += 1; }
    for i in b { bcount[(*i - b'a') as usize] += 1; }

    let mut result = i32::MAX;
    let mut asum = 0;
    let mut bsum = 0;
    for i in 0..25 {
        asum += acount[i];
        bsum += bcount[i];
        result = result.min(alen - acount[i] + blen - bcount[i]).min(alen - asum + bsum).min(blen - bsum + asum);
    }
    result.min(alen + blen - acount[25] - bcount[25])
}

fn main() {
    assert_eq!(min_characters(String::from("aba"), String::from("caa")), 2);
    assert_eq!(min_characters(String::from("dabadd"), String::from("cda")), 3);
}
