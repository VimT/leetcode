//! 最长快乐字符串


pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut cnt = [(a, b'a'), (b, b'b'), (c, b'c')];
    let mut result = vec![];
    loop {
        cnt.sort_unstable();
        let mut has_next = false;
        for (c, ch) in cnt.iter_mut().rev() {
            if *c <= 0 { break; }
            if result.len() >= 2 && result[result.len() - 2] == *ch && result[result.len() - 1] == *ch {
                continue;
            }
            has_next = true;
            result.push(*ch);
            *c -= 1;
            break;
        }
        if !has_next {
            break;
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(longest_diverse_string(1, 1, 7), String::from("ccbccacc"));
    assert_eq!(longest_diverse_string(7, 1, 0), String::from("aabaa"));
}
