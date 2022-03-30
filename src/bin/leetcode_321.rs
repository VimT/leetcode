//! 拼接最大数

pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
    // 从数组中获取指定长度的最大子序列，单调栈
    fn get_num(nums: &Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 { return vec![]; }
        let mut s = vec![0; k as usize];
        let mut top = -1;
        let len = nums.len();
        let mut remain = len as i32 - k;
        for i in 0..len {
            while top >= 0 && s[top as usize] < nums[i] && remain > 0 {
                top -= 1;
                remain -= 1;
            }
            if top < k - 1 {
                top += 1;
                s[top as usize] = nums[i];
            } else {
                remain -= 1;
            }
        }
        s
    }

    #[inline]
    fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut i1 = 0;
        let mut i2 = 0;
        let mut result = Vec::with_capacity(len1 + len2);
        while i1 < len1 && i2 < len2 {
            if nums1[i1..] > nums2[i2..] {
                result.push(nums1[i1]);
                i1 += 1;
            } else {
                result.push(nums2[i2]);
                i2 += 1;
            }
        }
        while i1 < len1 {
            result.push(nums1[i1]);
            i1 += 1;
        }
        while i2 < len2 {
            result.push(nums2[i2]);
            i2 += 1;
        }
        result
    }

    let len1 = nums1.len() as i32;
    let len2 = nums2.len() as i32;
    let mut result = vec![];
    for i in 0.max(k - len2)..=k.min(len1) {
        let n1 = get_num(&nums1, i);
        let n2 = get_num(&nums2, k - i);
        result = result.max(merge(n1, n2));
    }
    result
}

fn main() {
    assert_eq!(max_number(vec![6, 7], vec![6, 0, 4], 5), vec![6, 7, 6, 0, 4]);
    assert_eq!(max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5), vec![9, 8, 6, 5, 3]);
    assert_eq!(max_number(vec![3, 9], vec![8, 9], 3), vec![9, 8, 9]);
}
