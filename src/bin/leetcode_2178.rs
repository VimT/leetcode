//! 拆分成最多数目的偶整数之和

pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
    if final_sum & 1 == 1 { return vec![]; }
    let mut result = vec![];
    let mut left = final_sum;
    for i in (2..final_sum).step_by(2) {
        left -= i;
        if left <= i {
            left += i;
            break;
        }
        result.push(i);
    }
    result.push(left);
    result
}

fn main() {
    assert_eq!(maximum_even_split(12), vec![2, 4, 6]);
    assert_eq!(maximum_even_split(7), vec![]);
    assert_eq!(maximum_even_split(28), vec![2, 4, 6, 16]);
}
