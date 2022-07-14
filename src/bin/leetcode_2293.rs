//! 极大极小游戏


pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
    let mut new_nums = vec![0; nums.len() / 2];
    while nums.len() > 1 {
        for i in 0..new_nums.len() {
            if i & 1 == 0 {
                new_nums[i] = nums[2 * i].min(nums[2 * i + 1]);
            } else {
                new_nums[i] = nums[2 * i].max(nums[2 * i + 1]);
            }
        }
        nums = new_nums;
        new_nums = vec![0; nums.len() / 2]
    }
    nums[0]
}

fn main() {
    assert_eq!(min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
    assert_eq!(min_max_game(vec![3]), 3);
}
