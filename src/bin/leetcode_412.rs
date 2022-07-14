//! Fizz Buzz

pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n).map(|x| {
        if x % 3 == 0 && x % 5 == 0 {
            return String::from("FizzBuzz");
        } else if x % 3 == 0 {
            return String::from("Fizz");
        } else if x % 5 == 0 {
            return String::from("Buzz");
        } else {
            x.to_string()
        }
    }).collect()
}

fn main() {
    fn test(func: fn(n: i32) -> Vec<String>) {
        assert_eq!(func(3), vec!["1", "2", "Fizz"]);
        assert_eq!(func(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(func(15), vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]);
    }
    test(fizz_buzz);
}
