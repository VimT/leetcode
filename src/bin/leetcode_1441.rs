//! 用栈操作构建数组

pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
    let mut cur = 1;
    let mut result = Vec::with_capacity(target.len() * 2);
    for i in target {
        while i > cur {
            result.push(String::from("Push"));
            result.push("Pop".into());
            cur += 1;
        }
        result.push("Push".into());
        cur += 1;
    }
    result
}


fn main() {
    assert_eq!(build_array(vec![1, 3], 3), vec!["Push", "Push", "Pop", "Push"]);
    assert_eq!(build_array(vec![1, 2, 3], 3), vec!["Push", "Push", "Push"]);
    assert_eq!(build_array(vec![1, 2], 4), vec!["Push", "Push"]);
}
