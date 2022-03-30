//! 救生艇

pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();
    let len = people.len();
    let mut left = 0;
    let mut right = len - 1;
    let mut result = 0;
    while left < right {
        if people[right] + people[left] <= limit {
            result += 1;
            left += 1;
            right -= 1;
        } else {
            right -= 1;
            result += 1;
        }
    }
    if left == right { result += 1; }
    result
}

fn main() {
    assert_eq!(num_rescue_boats(vec![1, 2, 3], 3), 2);
    assert_eq!(num_rescue_boats(vec![1, 2], 3), 1);
    assert_eq!(num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    assert_eq!(num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
}
