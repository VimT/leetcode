//! 四数之和

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let n = nums.len();
    if n < 3 { return vec![]; }
    nums.sort_unstable();
    let mut ans = vec![];
    for i in 0..n {
        if i > 0 && nums[i] == nums[i - 1] { continue; }
        for j in i + 1..n - 2 {
            if j > i + 1 && nums[j] == nums[j - 1] { continue; }
            let mut l = j + 1;
            let mut r = n - 1;
            let min = nums[i] + nums[j] + nums[l] + nums[l + 1];
            if min > target { break; }
            let max = nums[i] + nums[j] + nums[r] + nums[r - 1];
            if max < target { continue; }
            while l < r {
                let s = nums[i] + nums[j] + nums[l] + nums[r];
                if s == target {
                    ans.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] { l += 1; }
                    while l < r && nums[r] == nums[r - 1] { r -= 1; }
                    l += 1;
                    r -= 1;
                } else if s > target { r -= 1; } else { l += 1; }
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(four_sum(vec![1, 0, -1, 0, -2, 2], 0), vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]);
    assert_eq!(four_sum(vec![2, 2, 2, 2, 2], 8), vec![vec![2, 2, 2, 2]]);
}
