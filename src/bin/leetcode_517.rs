//! 超级洗衣机

pub fn find_min_moves(machines: Vec<i32>) -> i32 {
    let sum: i32 = machines.iter().sum();
    let len = machines.len();
    if sum % len as i32 != 0 {
        return -1;
    }
    let avg = sum / len as i32;
    let mut result = 0;
    let mut s = 0;
    for num in machines {
        let num = num - avg;
        s += num;
        result = result.max(s.abs()).max(num);
    }
    return result;
}

fn main() {
    assert_eq!(find_min_moves(vec![1, 0, 5]), 3);
    assert_eq!(find_min_moves(vec![0, 3, 0]), 2);
    assert_eq!(find_min_moves(vec![0, 2, 0]), -1);
}
