//!  给植物浇水

pub fn watering_plants(mut plants: Vec<i32>, capacity: i32) -> i32 {
    let mut x = -1;
    let mut result = 0;
    let mut left = capacity;
    let len = plants.len();
    for i in 0..len {
        result += i as i32 - x;
        x = i as i32;
        loop {
            if left < plants[i] {
                plants[i] -= left;
                result += (i as i32 + 1) * 2;
                left = capacity;
            } else {
                left -= plants[i];
                break;
            }
        }
        if i < len - 1 && left < plants[i + 1] {
            result += i as i32 + 1;
            left = capacity;
            x = -1;
        }
    }
    result
}

fn main() {
    assert_eq!(watering_plants(vec![2, 2, 3, 3], 5), 14);
    assert_eq!(watering_plants(vec![1, 1, 1, 4, 2, 3], 4), 30);
    assert_eq!(watering_plants(vec![7, 7, 7, 7, 7, 7, 7], 8), 49);
}
