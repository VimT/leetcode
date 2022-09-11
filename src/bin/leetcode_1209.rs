//! 删除字符串中的所有相邻重复项 II

pub fn remove_duplicates(s: String, k: i32) -> String {
    let mut stk: Vec<(u8, i32)> = vec![];
    for &ch in s.as_bytes() {
        match stk.last_mut() {
            Some(last) if last.0 == ch => {
                last.1 += 1;
                if last.1 == k {
                    stk.pop();
                }
            }
            _ => {
                stk.push((ch, 1));
            }
        }
    }
    unsafe {
        String::from_utf8_unchecked(
            stk.into_iter()
                .map(|(ch, cnt)| vec![ch; cnt as usize])
                .flatten().collect(),
        )
    }
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> String) {
        assert_eq!(func(String::from("abcd"), 2), String::from("abcd"));
        assert_eq!(func(String::from("deeedbbcccbdaa"), 3), String::from("aa"));
        assert_eq!(func(String::from("pbbcggttciiippooaais"), 2), String::from("ps"));
    }
    test(remove_duplicates);
}
