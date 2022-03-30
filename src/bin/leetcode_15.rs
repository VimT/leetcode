//! 三数之和

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    if n < 3 { return vec![]; }
    nums.sort();
    let mut ans = vec![];
    for i in 0..n {
        if nums[i] > 0 { return ans; }
        if i > 0 && nums[i] == nums[i - 1] { continue; }
        let mut l = i + 1;
        let mut r = n - 1;
        while l < r {
            let s = nums[i] + nums[l] + nums[r];
            if s == 0 {
                ans.push(vec![nums[i], nums[l], nums[r]]);
                while l < r && nums[l] == nums[l + 1] { l += 1; }
                while l < r && nums[r] == nums[r - 1] { r -= 1; }
                l += 1;
                r -= 1;
            } else if s > 0 { r -= 1; } else { l += 1; }
        }
    }
    ans
}

fn main() {
    assert_eq!(three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    assert_eq!(three_sum(vec![]), vec![]);
    assert_eq!(three_sum(vec![0]), vec![]);
}
