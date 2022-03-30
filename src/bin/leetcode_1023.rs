//! 驼峰式匹配

use leetcode::svec;

pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    let p = pattern.as_bytes();
    let plen = p.len();
    queries.into_iter().map(|x| {
        let q = x.as_bytes();
        let qlen = q.len();
        let mut i = 0;
        let mut j = 0;
        while i < qlen && j < plen {
            if q[i] == p[j] {
                i += 1;
                j += 1;
            } else {
                if q[i].is_ascii_uppercase() {
                    return false;
                }
                i += 1;
            }
        }
        j == plen && q[i..].iter().find(|x| x.is_ascii_uppercase()).is_none()
    }).collect()
}

fn main() {
    assert_eq!(camel_match(svec!["FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"], String::from("FB")), vec![true, false, true, true, false]);
    assert_eq!(camel_match(svec!["FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"], String::from("FoBa")), vec![true, false, true, false, false]);
    assert_eq!(camel_match(svec!["FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"], String::from("FoBaT")), vec![false, true, false, false, false]);
}
