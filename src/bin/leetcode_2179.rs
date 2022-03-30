//! 统计数组中好三元组数目

/// 枚举y，y左侧有多少个比y小的数，y右侧有多少个比y大的数
pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let len = nums1.len();
    let mut p = vec![0; len];
    for (i, &x) in nums1.iter().enumerate() {
        p[x as usize] = i as i32;
    }
    let mut result = 0;
    // 树状数组
    let mut tree = vec![0; len + 1];
    for i in 1..len - 1 {
        let mut j = p[nums2[i - 1] as usize] + 1;
        while j <= len as i32 {
            tree[j as usize] += 1;
            j += j & -j;
        }
        let y = p[nums2[i] as usize];
        let mut less = 0;
        j = y;
        while j > 0 {
            less += tree[j as usize];
            j &= j - 1;
        }
        result += less as i64 * (len as i32 - 1 - y - (i as i32 - less)) as i64;
    }
    result
}

fn main() {
    assert_eq!(good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3]), 1);
    assert_eq!(good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3]), 4);
}
