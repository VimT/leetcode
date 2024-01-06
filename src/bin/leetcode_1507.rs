//! 转变日期格式

pub fn reformat_date(date: String) -> String {
    let date: Vec<&str> = date.split(" ").collect();
    let day: i32 = date[0].chars().take_while(|x| x.is_numeric()).collect::<String>().parse().unwrap();
    static MONTH: [&'static str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let month = MONTH.iter().position(|&x| x == date[1]).unwrap() + 1;
    format!("{}-{:02}-{:02}", date[2], month, day)
}

fn main() {
    fn test(func: fn(date: String) -> String) {
        assert_eq!(func(String::from("20th Oct 2052")), String::from("2052-10-20"));
        assert_eq!(func(String::from("6th Jun 1933")), String::from("1933-06-06"));
        assert_eq!(func(String::from("26th May 1960")), String::from("1960-05-26"));
    }
    test(reformat_date);
}
