//! 计算列车到站时间

pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
    return (arrival_time + delayed_time) % 24;
}

fn main() {
    fn test(func: fn(arrival_time: i32, delayed_time: i32) -> i32) {
        assert_eq!(func(15, 5), 20);
        assert_eq!(func(13, 11), 0);
    }
    test(find_delayed_arrival_time);
}
