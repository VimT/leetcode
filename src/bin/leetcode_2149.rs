//! 按符号重排数组

pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut zhen = 0;
    let mut fu = 0;
    let mut result = vec![0; len];
    for i in 0..len {
        if i & 1 == 0 {
            while nums[zhen] < 0 { zhen += 1; }
            result[i] = nums[zhen];
            zhen += 1;
        } else {
            while nums[fu] > 0 { fu += 1; }
            result[i] = nums[fu];
            fu += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(rearrange_array(vec![3, 1, -2, -5, 2, -4]), [3, -2, 1, -5, 2, -4]);
    assert_eq!(rearrange_array(vec![-1, 1]), [1, -1]);
}
