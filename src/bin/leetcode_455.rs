//! 分发饼干

pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort_unstable();
    s.sort_unstable();
    let mut si = 0;
    let mut result = 0;
    for people in g {
        while si < s.len() && s[si] < people { si += 1; }
        if si == s.len() { break; }
        result += 1;
        si += 1;
    }
    result
}

fn main() {
    assert_eq!(find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
    assert_eq!(find_content_children(vec![1, 2], vec![1, 2, 3]), 2);
}
