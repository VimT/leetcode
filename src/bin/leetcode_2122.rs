//! 还原原数组

pub fn recover_array(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let len = nums.len();
    for j in 1..=len / 2 {
        let k = (nums[j] - nums[0]) / 2;
        if k > 0 {
            let mut x = 0;
            let mut y = j;
            let mut result = Vec::with_capacity(len / 2);
            let mut vis = vec![false; len];
            vis[x] = true;
            vis[y] = true;
            for _ in 0..len / 2 {
                result.push(nums[x] + k);
                while x < len && vis[x] { x += 1; }
                if x < len { vis[x] = true; } else { break; }
                while y < len && (vis[y] || nums[y] != nums[x] + 2 * k) { y += 1; }
                if y < len { vis[y] = true; } else { break; }
            }
            if result.len() == len / 2 { return result; }
        }
    }
    vec![]
}

fn main() {
    assert_eq!(recover_array(vec![11, 6, 3, 4, 8, 7, 8, 7, 9, 8, 9, 10, 10, 2, 1, 9]), vec![2, 3, 7, 8, 8, 9, 9, 10]);
    assert_eq!(recover_array(vec![2, 10, 6, 4, 8, 12]), vec![3, 7, 11]);
    assert_eq!(recover_array(vec![1, 1, 3, 3]), vec![2, 2]);
    assert_eq!(recover_array(vec![5, 435]), vec![220]);
}
