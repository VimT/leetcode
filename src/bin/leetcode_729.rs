//! 我的日程安排表 I

struct MyCalendar {
    ranges: Vec<(i32, i32)>,
}


impl MyCalendar {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn book(&mut self, start: i32, mut end: i32) -> bool {
        end -= 1;
        for &range in &self.ranges {
            if start <= range.1 && range.0 <= end {
                return false;
            }
        }
        self.ranges.push((start, end));
        true
    }
}

fn main() {
    let mut c = MyCalendar::new();
    assert_eq!(c.book(10, 20), true); // return True
    assert_eq!(c.book(15, 25), false); // return False ，这个日程安排不能添加到日历中，因为时间 15 已经被另一个日程安排预订了。
    assert_eq!(c.book(20, 30), true); // return True ，这个日程安排可以添加到日历中，因为第一个日程安排预订的每个时间都小于 20 ，且不包含时间 20 。

    assert_eq!(c.book(48, 50), true);
    assert_eq!(c.book(49, 49), false);
}