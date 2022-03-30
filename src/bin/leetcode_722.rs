//! 删除注释

use leetcode::svec;

pub fn remove_comments(source: Vec<String>) -> Vec<String> {
    let mut result = Vec::with_capacity(source.len());
    let mut in_block = false;
    let mut new_line = vec![];
    for line in source {
        let mut i = 0;
        let s = line.as_bytes();
        let len = s.len();
        if !in_block { new_line.clear(); }
        while i < len {
            if &s[i..(i + 2).min(len)] == b"/*" && !in_block {
                in_block = true;
                i += 1;
            } else if &s[i..(i + 2).min(len)] == b"*/" && in_block {
                in_block = false;
                i += 1;
            } else if !in_block && &s[i..(i + 2).min(len)] == b"//" {
                break;
            } else if !in_block {
                new_line.push(s[i]);
            }
            i += 1;
        }
        if !new_line.is_empty() && !in_block {
            unsafe { result.push(String::from_utf8_unchecked(new_line.clone())) }
        }
    }
    result
}

fn main() {
    assert_eq!(remove_comments(svec!["a/*comment", "line", "more_comment*/b"]), ["ab"]);
    assert_eq!(remove_comments(svec!["main() {", "   int x = 1; // Its comments here", "   x++;", "   cout << x;", "   //return x;", "   x--;", "}"]), ["main() {", "   int x = 1; ", "   x++;", "   cout << x;", "   ", "   x--;", "}"]);
    assert_eq!(remove_comments(svec!["/*Test program */", "int main()", "{ ", "  // variable declaration ", "int a, b, c;", "/* This is a test", "   multiline  ", "   comment for ", "   testing */", "a = b + c;", "}"]), vec!["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"]);
}
