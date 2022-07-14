//! 字符串中最大的 3 位相同数字

pub fn largest_good_integer(num: String) -> String {
    let mut result = vec![];
    for i in num.as_bytes().windows(3) {
        if i[0] == i[1] && i[1] == i[2] {
            if result.is_empty() || i[0] > result[0] {
                result = i.to_vec();
            }
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(largest_good_integer(String::from("677713333999")), "999");
    assert_eq!(largest_good_integer(String::from("42352338")), "");
}
