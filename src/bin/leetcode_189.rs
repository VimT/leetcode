//! 轮转数组

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = k as usize % len;
    if k == 0 { return; }
    nums.reverse();

    (&mut nums[0..k]).reverse();
    (&mut nums[k..len]).reverse();
}


fn main() {
    fn test(func: fn(nums: &mut Vec<i32>, k: i32)) {
        let help = |mut nums: Vec<i32>, k: i32| -> Vec<i32> {
            func(&mut nums, k);
            nums
        };
        assert_eq!(help(vec![1, 2, 3, 4, 5, 6, 7], 3), [5, 6, 7, 1, 2, 3, 4]);
        assert_eq!(help(vec![-1, -100, 3, 99], 2), [3, 99, -1, -100]);
    }
    test(rotate);
}
