//! 找出数组中的美丽下标 II

use leetcode::algorithm::kmp_all;

pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
    let mut result = vec![];
    let b_list = kmp_all(s.as_bytes(), b.as_bytes());
    let mut j = 0;
    for i in kmp_all(s.as_bytes(), a.as_bytes()) {
        while j < b_list.len() && b_list[j] <= i + k as usize {
            if b_list[j] as i32 >= i as i32 - k {
                result.push(i as i32);
                break;
            }
            j += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(s: String, a: String, b: String, k: i32) -> Vec<i32>) {
        assert_eq!(func(String::from("ababababazzabababb"), String::from("aba"), String::from("bb"), 10), vec![6, 11, 13]);
        assert_eq!(func(String::from("isawsquirrelnearmysquirrelhouseohmy"), String::from("my"), String::from("squirrel"), 15), vec![16, 33]);
        assert_eq!(func(String::from("abcd"), String::from("a"), String::from("a"), 4), vec![0]);
    }
    test(beautiful_indices);
}
