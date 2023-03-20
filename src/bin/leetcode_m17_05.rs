//! 字母与数字


use std::collections::HashMap;

pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
    let mut cnt = 0;
    let len = array.len();
    let mut cnt_map = HashMap::with_capacity(len);
    cnt_map.insert(0, 0);
    let mut max_len = 0;
    let mut start = 0;
    for (i, num) in array.iter().enumerate() {
        if num.as_bytes()[0].is_ascii_digit() {
            cnt -= 1;
        } else {
            cnt += 1;
        }
        if let Some(&j) = cnt_map.get(&cnt) {
            if i - j + 1> max_len {
                max_len = i - j + 1;
                start = j;
            }
        } else {
            cnt_map.insert(cnt, i + 1);
        }
    }
    array[start..start + max_len].to_vec()
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(array: Vec<String>) -> Vec<String>) {
        assert_eq!(func(svec!["A","1","B","C","D","2","3","4","E","5","F","G","6","7","H","I","J","K","L","M"]), svec!["A","1","B","C","D","2","3","4","E","5","F","G","6","7"]);
        assert_eq!(func(svec!["A","A"]), svec![]);
        assert_eq!(func(svec!["A","1"]), svec!["A","1"]);
    }
    test(find_longest_subarray);
}
