//! 最接近的三数之和

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut ans = nums[0] + nums[1] + nums[2];
    nums.sort();
    for i in 0..n {
        let mut l = i + 1;
        let mut r = n - 1;
        while l < r {
            let s = nums[i] + nums[l] + nums[r];
            if (s - target).abs() < (target - ans).abs() {
                ans = s;
            }
            if s > target { r -= 1; } else if s < target { l += 1; } else { return s; }
        }
    }
    ans
}

fn main() {
    assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
}
