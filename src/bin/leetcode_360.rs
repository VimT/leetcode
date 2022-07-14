//! 有序转化数组

/// 双指针，针对a的正负，分情况讨论
pub fn sort_transformed_array(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
    let len = nums.len();
    let mut l = 0;
    let mut r = len;
    let mut result = vec![0; len];
    if a > 0 {
        let mut i = len;
        while l < r {
            let lval = a * nums[l] * nums[l] + b * nums[l] + c;
            let rval = a * nums[r - 1] * nums[r - 1] + b * nums[r - 1] + c;
            if lval <= rval {
                result[i - 1] = rval;
                i -= 1;
                r -= 1;
            } else {
                result[i - 1] = lval;
                i -= 1;
                l += 1;
            }
        }
    } else {
        let mut i = 0;
        while l < r {
            let lval = a * nums[l] * nums[l] + b * nums[l] + c;
            let rval = a * nums[r - 1] * nums[r - 1] + b * nums[r - 1] + c;
            if lval <= rval {
                result[i] = lval;
                i += 1;
                l += 1;
            } else {
                result[i] = rval;
                i += 1;
                r -= 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32>) {
        assert_eq!(func(vec![-4, -2, 2, 4], 1, 3, 5), vec![3, 9, 15, 33]);
        assert_eq!(func(vec![-4, -2, 2, 4], -1, 3, 5), vec![-23, -5, 1, 7]);
    }
    test(sort_transformed_array);
}
