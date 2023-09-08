//! 文件夹操作日志搜集器

pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut depth = 0;
    for log in logs {
        match log.as_str() {
            "../" => if depth > 0 { depth -= 1; }
            "./" => (),
            _ => depth += 1,
        }
    }
    depth
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(logs: Vec<String>) -> i32) {
        assert_eq!(func(svec!["d1/","d2/","../","d21/","./"]), 2);
        assert_eq!(func(svec!["d1/","d2/","./","d3/","../","d31/"]), 3);
        assert_eq!(func(svec!["d1/","../","../","../"]), 0);
    }
    test(min_operations);
}
