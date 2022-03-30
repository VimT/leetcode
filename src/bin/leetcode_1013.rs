//! 将数组分成和相等的三个部分

pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
    let sum: i32 = arr.iter().sum();
    if sum % 3 != 0 { return false; }
    let part = sum / 3;
    let mut cursum = 0;
    let len = arr.len();
    let mut i = 0;
    while i < len {
        cursum += arr[i];
        i += 1;
        if cursum == part {
            break;
        }
    }
    if cursum != part { return false; }
    cursum = 0;
    while i < len {
        cursum += arr[i];
        i += 1;
        if cursum == part {
            break;
        }
    }
    cursum == part && i < len
}

fn main() {
    assert_eq!(can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]), true);
    assert_eq!(can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]), false);
    assert_eq!(can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]), true);
}
