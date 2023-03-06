//! 左右元素和的差值

pub fn left_rigth_difference(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut presum = vec![0; len];
    for i in 0..len - 1 {
        presum[i + 1] = presum[i] + nums[i];
    }
    let mut sufsum = vec![0; len];
    for i in (1..len).rev() {
        sufsum[i - 1] = sufsum[i] + nums[i];
    }
    (0..len).map(|x| (presum[x] - sufsum[x]).abs()).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![10, 4, 8, 3]), vec![15, 1, 11, 22]);
        assert_eq!(func(vec![1]), vec![0]);
    }
    test(left_rigth_difference);
}
