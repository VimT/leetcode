//! 生成不含相邻零的二进制字符串

pub fn valid_strings(n: i32) -> Vec<String> {
    let mut result = vec!["0".to_string(), "1".to_string()];
    for _ in 1..n {
        let mut new = vec![];
        for item in result {
            if item.as_bytes().last().copied().unwrap() == b'0' {
                new.push(format!("{}1", item));
            } else {
                new.push(format!("{}0", item));
                new.push(format!("{}1", item));
            }
        }
        result = new;
    }
    result
}

/// 位运算做法
pub fn valid_strings2(n: i32) -> Vec<String> {
    let mut result = vec![];
    let mask = (1 << n) - 1;
    for i in 0..(1 << n) {
        let x = mask ^ i;
        // 判断是否相邻有 0
        if x & (x >> 1) == 0 {
            result.push(format!("{:0width$b}", i, width = n as usize));
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32) -> Vec<String>) {
        assert_eq!(func(3), vec!["010", "011", "101", "110", "111"]);
        assert_eq!(func(1), vec!["0", "1"]);
    }
    test(valid_strings);
    test(valid_strings2);
}
