//! 价格减免

pub fn discount_prices(sentence: String, discount: i32) -> String {
    let multi = (100 - discount) as f64 / 100.;
    sentence.split(' ').map(|x| {
        if x.starts_with("$") {
            if let Ok(num) = x[1..].parse::<u64>() {
                return format!("${:.2}", num as f64 * multi);
            }
        }
        x.to_string()
    }).collect::<Vec<String>>().join(" ")
}

fn main() {
    fn test(func: fn(sentence: String, discount: i32) -> String) {
        assert_eq!(func(String::from("there are $1 $2 and 5$ candies in the shop"), 50), String::from("there are $0.50 $1.00 and 5$ candies in the shop"));
        assert_eq!(func(String::from("1 2 $3 4 $5 $6 7 8$ $9 $10$"), 100), String::from("1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$"));
    }
    test(discount_prices);
}
