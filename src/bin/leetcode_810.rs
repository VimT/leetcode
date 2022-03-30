//! 黑板异或游戏

pub fn xor_game(nums: Vec<i32>) -> bool {
    if nums.len() & 1 == 0 {
        return true;
    }
    let mut xor = 0;
    for num in nums {
        xor ^= num;
    }
    xor == 0
}

fn main() {
    assert_eq!(xor_game(vec![1, 1, 2]), false);
    assert_eq!(xor_game(vec![0, 1]), true);
    assert_eq!(xor_game(vec![1, 2, 3]), true);
}
