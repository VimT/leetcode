//! 下一个更大元素 III

pub fn next_greater_element(n: i32) -> i32 {
    let mut list = vec![];
    let mut num = n as i64;
    while num > 0 {
        list.push(num % 10);
        num /= 10;
    }
    list.reverse();
    let len = list.len();
    for i in (0..len - 1).rev() {
        let mut min = i64::MAX;
        let mut min_idx = 0;
        for j in i + 1..len {
            if list[j] < min && list[j] > list[i] {
                if i == 0 && list[j] == 0 {
                    continue;
                }
                min = list[j];
                min_idx = j;
            }
        }
        if min_idx > 0 && min > list[i] {
            list.swap(i, min_idx);
            list[i + 1..].sort_unstable();
            let mut num = 0;
            for i in 0..len {
                num = num * 10 + list[i];
            }
            if num > i32::MAX as i64 {
                return -1;
            }
            return num as i32;
        }
    }
    -1
}

fn main() {
    assert_eq!(next_greater_element(2147483486), -1);
    assert_eq!(next_greater_element(230241), 230412);
    assert_eq!(next_greater_element(100000000), -1);
    assert_eq!(next_greater_element(12), 21);
    assert_eq!(next_greater_element(21), -1);
    assert_eq!(next_greater_element(1), -1);
    assert_eq!(next_greater_element(987654321), -1);
    assert_eq!(next_greater_element(123456789), 123456798);
    assert_eq!(next_greater_element(100000002), 100000020);
}
