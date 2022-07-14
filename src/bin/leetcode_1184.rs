//! 公交站间的距离

pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let len = distance.len();
    let mut cur = start;
    let mut forward = 0;
    while cur != destination {
        forward += distance[cur as usize];
        cur = (cur + 1) % len as i32;
    }

    cur = start;
    let mut back = 0;
    while cur != destination {
        cur = (cur - 1 + len as i32) % len as i32;
        back += distance[cur as usize];
    }

    forward.min(back)
}

fn main() {
    fn test(func: fn(distance: Vec<i32>, start: i32, destination: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4], 0, 1), 1);
        assert_eq!(func(vec![1, 2, 3, 4], 0, 2), 3);
        assert_eq!(func(vec![1, 2, 3, 4], 0, 3), 4);
    }
    test(distance_between_bus_stops);
}
