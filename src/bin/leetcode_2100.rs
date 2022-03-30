//! 适合打劫银行的日子

pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
    let len = security.len();
    let mut right = vec![1; len];
    let mut left = vec![1; len];
    for i in 1..len {
        if security[i] <= security[i - 1] {
            left[i] = left[i - 1] + 1;
        }
    }
    for i in (0..len - 1).rev() {
        if security[i] <= security[i + 1] {
            right[i] = right[i + 1] + 1;
        }
    }
    let time = time as usize;
    let mut result = vec![];
    for i in 0..len {
        if left[i] > time && right[i] > time {
            result.push(i as i32);
        }
    }
    result
}

fn main() {
    assert_eq!(good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2), vec![2, 3]);
    assert_eq!(good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 0), vec![0, 1, 2, 3, 4]);
    assert_eq!(good_days_to_rob_bank(vec![1, 2, 3, 4, 5, 6], 2), vec![]);
    assert_eq!(good_days_to_rob_bank(vec![1], 5), vec![]);
}
