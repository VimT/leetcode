//! 两个有序数组的第 K 小乘积

pub fn kth_smallest_product(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i64) -> i64 {
    fn cal(a: &Vec<i64>, b: &Vec<i64>, target: i64) -> i64 {
        let mut result = 0;
        for &x in a {
            if x > 0 {
                let k = b.partition_point(|&y| x * y <= target);
                result += k;
            } else if x < 0 {
                let k = b.partition_point(|&y| x * y > target);
                result += b.len() - k;
            } else if target >= 0 {
                result += b.len();
            }
        }
        result as i64
    }
    if nums1.len() > nums2.len() {
        std::mem::swap(&mut nums1, &mut nums2);
    }

    let a: Vec<i64> = nums1.into_iter().map(|x| x as i64).collect();
    let b: Vec<i64> = nums2.into_iter().map(|x| x as i64).collect();
    let mut left = -1e11 as i64;
    let mut right = 1e11 as i64;
    while left < right {
        let mid = (left + right) >> 1;
        let num = cal(&a, &b, mid);
        if num >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
        println!("{}", mid);
    }
    left
}

fn main() {
    assert_eq!(kth_smallest_product(vec![-10, -9, -8, -5, -3, -2, 1, 2, 4, 8],
                                    vec![-9, -8, -8, -4, -4, -3, -1, 0, 4],
                                    73), 32);
    assert_eq!(kth_smallest_product(vec![1, 6], vec![-10, -10, -5, -4, -3, -1], 9), -5);
    assert_eq!(kth_smallest_product(vec![-2, -1, 0, 1, 2], vec![-3, -1, 2, 4, 5], 3), -6);
    assert_eq!(kth_smallest_product(vec![-4, -2, 0, 3], vec![2, 4], 6), 0);
    assert_eq!(kth_smallest_product(vec![2, 5], vec![3, 4], 2), 8);
}

