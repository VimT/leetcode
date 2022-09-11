//! 数组能形成多少数对

pub fn number_of_pairs(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut i = 1;
    let mut same = 0;
    let mut not_same = 0;
    while i < nums.len() {
        if nums[i] == nums[i - 1] {
            same += 1;
            i += 2;
        } else {
            not_same += 1;
            i += 1;
        }
    }
    if i == nums.len() {not_same += 1;}
    vec![same, not_same]
}

fn main() {
    assert_eq!(number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]), [3, 1]);
    assert_eq!(number_of_pairs(vec![1, 1]), [1, 0]);
    assert_eq!(number_of_pairs(vec![0]), [0, 1]);
    assert_eq!(number_of_pairs(vec![1,2,3,4,5]), [0, 5]);
}