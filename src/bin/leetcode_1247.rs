//! 交换字符使得字符串相同

pub fn minimum_swap(s1: String, s2: String) -> i32 {
    if s1.len() != s2.len() { return -1; }
    let len = s1.len();
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let s1x = s1.iter().filter(|&&x| x == b'x').count();
    let s2x = s2.iter().filter(|&&x| x == b'x').count();
    if (s1x + s2x) & 1 == 1 {
        return -1;
    }
    let mut result = 0;
    let mut xy = 0;
    let mut yx = 0;
    for i in 0..len {
        if s1[i] != s2[i] {
            if s1[i] == b'x' {
                xy += 1;
            } else {
                yx += 1;
            }
        }
    }
    result += xy / 2;
    result += yx / 2;
    if xy & 1 == 1 {
        result += 2;
    }
    result
}


fn main() {
    assert_eq!(minimum_swap(String::from("xx"), String::from("yy")), 1);
    assert_eq!(minimum_swap(String::from("xy"), String::from("yx")), 2);
    assert_eq!(minimum_swap(String::from("xx"), String::from("xy")), -1);
}
