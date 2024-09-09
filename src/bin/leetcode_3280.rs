//! 将日期转换为二进制表示

pub fn convert_date_to_binary(date: String) -> String {
    date.split('-').map(|x| {
        let x = x.parse::<i32>().unwrap();
        format!("{:0b}", x)
    }).collect::<Vec<String>>().join("-")
}

fn main() {
    fn test(func: fn(date: String) -> String) {
        assert_eq!(func(String::from("2080-02-29")), String::from("100000100000-10-11101"));
        assert_eq!(func(String::from("1900-01-01")), String::from("11101101100-1-1"));
    }
    test(convert_date_to_binary);
}
