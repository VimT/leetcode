//! 将标题首字母大写

pub fn capitalize_title(title: String) -> String {
    title.split(' ').map(|x| {
        if x.len() <= 2 { return x.to_ascii_lowercase(); } else {
            let mut x = x.to_ascii_lowercase();
            unsafe { x.as_bytes_mut()[0].make_ascii_uppercase(); }
            x
        }
    }).collect::<Vec<String>>().join(" ")
}

fn main() {
    assert_eq!(capitalize_title(String::from("capiTalIze tHe titLe")), String::from("Capitalize The Title"));
    assert_eq!(capitalize_title(String::from("First leTTeR of EACH Word")), String::from("First Letter of Each Word"));
    assert_eq!(capitalize_title(String::from("i lOve leetcode")), String::from("i Love Leetcode"));
}
