//! 找出对应 LCP 矩阵的字符串


pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
    let len = lcp.len();
    let mut result = vec![0; len];
    let mut ch = b'a';
    for i in 0..len {
        if result[i] == 0 {
            if ch > b'z' {
                return String::new();
            }
            result[i] = ch;
            for j in i + 1..len {
                if lcp[i][j] > 0 {
                    result[j] = ch;
                }
            }
            ch += 1;
        }
    }
    for i in (0..len).rev() {
        for j in (0..len).rev() {
            let mut r = 0;
            if result[i] == result[j] {
                r = if i + 1 < len && j + 1 < len { lcp[i + 1][j + 1] } else { 0 } + 1;
            }
            if r != lcp[i][j] {
                return String::new();
            }
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(lcp: Vec<Vec<i32>>) -> String) {
        assert_eq!(func(vec![vec![4, 0, 2, 0], vec![0, 3, 0, 1], vec![2, 0, 2, 0], vec![0, 1, 0, 1]]), String::from("abab"));
        assert_eq!(func(vec![vec![2, 0], vec![0, 1]]), String::from("ab"));
        assert_eq!(func(vec![vec![4, 1, 1, 1], vec![1, 3, 1, 1], vec![1, 1, 2, 1], vec![1, 1, 1, 1]]), String::from(""));
        assert_eq!(func(vec![vec![4, 3, 2, 1], vec![3, 3, 2, 1], vec![2, 2, 2, 1], vec![1, 1, 1, 1]]), String::from("aaaa"));
        assert_eq!(func(vec![vec![4, 3, 2, 1], vec![3, 3, 2, 1], vec![2, 2, 2, 1], vec![1, 1, 1, 3]]), String::from(""));
    }
    test(find_the_string);
}
