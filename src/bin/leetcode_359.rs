//! 日志速率限制器

use std::collections::HashMap;

struct Logger {
    seen: HashMap<String, i32>,
}


impl Logger {
    fn new() -> Self {
        Self { seen: HashMap::new() }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(&last) = self.seen.get(&message) {
            if timestamp - last < 10 {
                return false;
            }
        }
        self.seen.insert(message, timestamp);
        true
    }
}


fn main() {
    let mut logger = Logger::new();
    assert_eq!(logger.should_print_message(1, String::from("foo")), true);  // 返回 true ，下一次 "foo" 可以打印的时间戳是 1 + 10 = 11
    assert_eq!(logger.should_print_message(2, String::from("bar")), true);  // 返回 true ，下一次 "bar" 可以打印的时间戳是 2 + 10 = 12
    assert_eq!(logger.should_print_message(3, String::from("foo")), false);  // 3 < 11 ，返回 false
    assert_eq!(logger.should_print_message(8, String::from("bar")), false);  // 8 < 12 ，返回 false
    assert_eq!(logger.should_print_message(10, String::from("foo")), false); // 10 < 11 ，返回 false
    assert_eq!(logger.should_print_message(11, String::from("foo")), true); // 11 >= 11 ，返回 true ，下一次 "foo" 可以打印的时间戳是 11 + 10 = 21
}
