//! 最大交换

pub fn maximum_swap(num: i32) -> i32 {
    if num == 0 { return 0; }
    let mut v = vec![];
    let mut p = num;
    while p > 0 {
        v.push(p % 10);
        p /= 10;
    }
    v.reverse();
    let len = v.len();
    let mut right_max = vec![len - 1; len];
    let mut p = len - 1;
    for i in (0..len).rev() {
        right_max[i] = p;
        if v[i] > v[p] {
            p = i;
        }
    }
    for i in 0..len {
        if v[i] < v[right_max[i]] {
            v.swap(i, right_max[i]);
            break;
        }
    }
    let mut result = 0;
    for i in 0..len {
        result = result * 10 + v[i];
    }
    result
}

fn main() {
    assert_eq!(maximum_swap(98368), 98863);
    assert_eq!(maximum_swap(1993), 9913);
    assert_eq!(maximum_swap(2736), 7236);
    assert_eq!(maximum_swap(9973), 9973);
}
