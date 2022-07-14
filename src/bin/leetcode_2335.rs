//! 装满杯子需要的最短总时长

pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
    let mut result = 0;
    amount.sort_unstable();
    while amount[1] > 0 && amount[2] > 0 {
        result += 1;
        amount[1] -= 1;
        amount[2] -= 1;
        amount.sort_unstable();
    }
    result + amount[2]
}

fn main() {
    assert_eq!(fill_cups(vec![1, 4, 2]), 4);
    assert_eq!(fill_cups(vec![5, 4, 4]), 7);
    assert_eq!(fill_cups(vec![5, 0, 0]), 5);
}
