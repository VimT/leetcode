//! Excel 表列序号

pub fn title_to_number(s: String) -> i32 {
    let mut ans = 0;
    let _e = 0;
    let chars = s.chars().rev().collect::<Vec<char>>();

    for (index, char) in chars.iter().enumerate() {
        ans += ((*char as u8 - b'A' + 1) as i32) * 26i32.pow(index as u32)
    }
    ans
}

fn main() {
    assert_eq!(title_to_number(String::from("A")), 1);
    assert_eq!(title_to_number(String::from("AB")), 28);
    assert_eq!(title_to_number(String::from("ZY")), 701);
}
