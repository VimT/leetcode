//! 单调递增的数字

pub fn monotone_increasing_digits(n: i32) -> i32 {
    let mut num = n;
    let mut v = vec![];
    while num > 0 {
        v.push(num % 10);
        num /= 10;
    }
    v.reverse();
    let len = v.len();
    let mut end = len;
    loop {
        let mut ok = true;
        for i in 1..end {
            if v[i] < v[i - 1] {
                ok = false;
                v[i - 1] -= 1;
                for j in i..end {
                    v[j] = 9;
                }
                end = i;
            }
        }
        if ok { break; }
    }

    for wei in v {
        num = num * 10 + wei;
    }
    num
}

fn main() {
    assert_eq!(monotone_increasing_digits(10), 9);
    assert_eq!(monotone_increasing_digits(1234), 1234);
    assert_eq!(monotone_increasing_digits(332), 299);
}
