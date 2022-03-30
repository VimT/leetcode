//! K 次取反后最大化的数组和

pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
    let len = nums.len();
    nums.sort_unstable_by_key(|x| -x.abs());
    for i in &mut nums {
        if *i < 0 {
            *i = -*i;
            k -= 1;
            if k == 0 {
                break;
            }
        }
    }
    if k > 0 && k & 1 == 1 {
        nums[len - 1] = -nums[len - 1];
    }
    nums.into_iter().sum()
}

fn main() {
    assert_eq!(largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    assert_eq!(largest_sum_after_k_negations(vec![3, -1, 0, 2], 3), 6);
    assert_eq!(largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2), 13);
}
