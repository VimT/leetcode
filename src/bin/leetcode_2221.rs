//! 数组的三角和

pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
    let mut len = nums.len();
    while len > 1 {
        for i in 0..len - 1 {
            nums[i] = (nums[i] + nums[i + 1]) % 10;
        }
        len -= 1;
    }
    nums[0]
}

fn main() {
    assert_eq!(triangular_sum(vec![1, 2, 3, 4, 5]), 8);
    assert_eq!(triangular_sum(vec![5]), 5);
}
