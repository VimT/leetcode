//! 按奇偶性交换后的最大数字

pub fn largest_integer(mut num: i32) -> i32 {
    let mut s = vec![];
    while num > 0 {
        s.push(num % 10);
        num /= 10;
    }
    s.reverse();
    let mut clone = s.clone();
    clone.sort_unstable_by_key(|x| -*x);
    let mut i = 0;
    let mut j = 0;
    for k in 0..s.len() {
        if s[k] & 1 == 1 {
            while clone[i] & 1 != 1 { i += 1; }
            s[k] = clone[i];
            i += 1;
        } else {
            while clone[j] & 1 != 0 { j += 1; }
            s[k] = clone[j];
            j += 1;
        }
    }
    let mut result = 0;
    for i in s {
        result = result * 10 + i;
    }
    result
}

fn main() {
    fn test(func: fn(num: i32) -> i32) {
        assert_eq!(func(1234), 3412);
        assert_eq!(func(65875), 87655);
    }
    test(largest_integer);
}
