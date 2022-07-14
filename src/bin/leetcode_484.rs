//! 寻找排列

/// 对于连续的D，翻转
pub fn find_permutation(s: String) -> Vec<i32> {
    let s = s.as_bytes();
    let mut result: Vec<i32> = (1..=s.len() as i32 + 1).collect();
    let mut i = 0;
    while i < s.len() {
        while i < s.len() && s[i] == b'I' {
            i += 1;
        }
        let mut j = i;
        while j < s.len() && s[j] == b'D' {
            j += 1;
        }
        result[i..=j].reverse();
        i = j;
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> Vec<i32>) {
        assert_eq!(func(String::from("I")), vec![1, 2]);
        assert_eq!(func(String::from("DI")), vec![2, 1, 3]);
    }
    test(find_permutation);
}
