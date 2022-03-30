//! 简化路径

pub fn simplify_path(path: String) -> String {
    let mut ans = vec![];
    for i in path.split("/") {
        match i {
            ".." => { ans.pop(); }
            "." => (),
            "" => (),
            v => { ans.push(v) }
        }
    }
    "/".to_string() + &ans.join("/")
}

fn main() {
    assert_eq!(simplify_path(String::from("/home/")), String::from("/home"));
    assert_eq!(simplify_path(String::from("/../")), String::from("/"));
    assert_eq!(simplify_path(String::from("/home//foo/")), String::from("/home/foo"));
}
