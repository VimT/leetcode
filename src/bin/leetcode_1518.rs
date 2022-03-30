//! 换酒问题


pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut result = 0;
    let mut empty = 0;
    while num_bottles > 0 {
        result += num_bottles;
        empty += num_bottles;
        num_bottles = empty / num_exchange;
        empty %= num_exchange;
    }
    result
}

fn main() {
    assert_eq!(num_water_bottles(9, 3), 13);
    assert_eq!(num_water_bottles(15, 4), 19);
    assert_eq!(num_water_bottles(5, 5), 6);
    assert_eq!(num_water_bottles(2, 3), 2);
}
