//! 坐上公交的最晚时间

pub fn latest_time_catch_the_bus(mut buses: Vec<i32>, mut passengers: Vec<i32>, capacity: i32) -> i32 {
    buses.sort_unstable();
    passengers.sort_unstable();
    let capacity = capacity as usize;
    let mut pi = 0;
    let mut result = 1;
    for bus in buses {
        let mut rest = capacity;
        while rest > 0 && pi < passengers.len() && passengers[pi] <= bus {
            // 取而代之
            result = passengers[pi];
            pi += 1;
            rest -= 1;
        }
        if rest > 0 {
            // 还有空位，最后一刻坐上去
            result = bus;
        }
    }
    while pi > 0 && result == passengers[pi - 1] {
        result -= 1;
        pi -= 1;
    }
    result
}

fn main() {
    fn test(func: fn(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32) {
        assert_eq!(func(vec![10, 20], vec![2, 17, 18, 19], 2), 16);
        assert_eq!(func(vec![20, 30, 10], vec![19, 13, 26, 4, 25, 11, 21], 2), 20);
    }
    test(latest_time_catch_the_bus);
}
