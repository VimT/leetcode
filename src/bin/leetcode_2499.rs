//! 让数组不相等的最小总代价

/// 分类讨论
/// 重复的数字里面，出现次数最多的数x。 如果出现次数 > cnt/2，则需要和非重复的数交换。否则可以在重复数内部交换消化
pub fn minimum_total_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let len = nums1.len();
    let mut cnt = 0;
    let mut vis = vec![0; len + 1];
    let mut x = 0;
    let mut result = 0;
    for i in 0..len {
        let val = nums1[i];
        if val == nums2[i] {
            cnt += 1;
            vis[val as usize] += 1;
            if vis[val as usize] > vis[x as usize] { x = val; }
            result += i as i64;
        }
    }
    let mut fd = vis[x as usize] * 2 - cnt;
    for i in 0..len {
        if fd <= 0 { break; }
        let val = nums1[i];
        if val != nums2[i] && nums2[i] != x && nums1[i] != x {
            result += i as i64;
            fd -= 1;
        }
    }
    if fd > 0 { -1 } else { result }
}

fn main() {
    assert_eq!(minimum_total_cost(vec![1, 2], vec![2, 1]), 0);
    assert_eq!(minimum_total_cost(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]), 10);
    assert_eq!(minimum_total_cost(vec![2, 2, 2, 1, 3], vec![1, 2, 2, 3, 3]), 10);
    assert_eq!(minimum_total_cost(vec![1, 2, 2], vec![1, 2, 2]), -1);
}
