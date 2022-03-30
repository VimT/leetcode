//! 航班预订统计

pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut diff = vec![0; n as usize + 1];
    for book in bookings {
        diff[book[0] as usize - 1] += book[2];
        diff[book[1] as usize] -= book[2];
    }
    let mut result = vec![0; n as usize];
    let mut cur = 0;
    for i in 0..n as usize {
        cur += diff[i];
        result[i] = cur;
    }
    result
}

fn main() {
    fn test(func: fn(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5), vec![10, 55, 45, 25, 25]);
        assert_eq!(func(vec![vec![1, 2, 10], vec![2, 2, 15]], 2), vec![10, 25]);
    }
    test(corp_flight_bookings);
}
