//! 摆动序列


/// write by self
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    // num, last one
    let mut up = (1, i32::MIN);
    let mut down = (1, i32::MAX);

    for i in nums {
        if up.0 > down.0 {
            if i < up.1 {
                down = (up.0 + 1, i);
            }
            if i > down.1 {
                up = (down.0 + 1, i);
            }
        } else {
            if i > down.1 {
                up = (down.0 + 1, i);
            }
            if i < up.1 {
                down = (up.0 + 1, i);
            }
        }

        if i < down.1 {
            down.1 = i;
        }
        if i > up.1 {
            up.1 = i;
        }
    }
    up.0.max(down.0)
}

/// 方法三：贪心
/// 不断地交错选择「峰」与「谷」，可以使得该序列尽可能长
fn wiggle_max_length_greedy(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len < 2 { return len as i32; }
    let mut prevdiff = nums[1] - nums[0];
    let mut ret = if prevdiff != 0 { 2 } else { 1 };
    for i in 2..len {
        let diff = nums[i] - nums[i - 1];
        if (diff > 0 && prevdiff <= 0) || (diff < 0 && prevdiff >= 0) {
            ret += 1;
            prevdiff = diff;
        }
    }
    ret
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(func(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]), 7);
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 2);
    }
    test(wiggle_max_length);
    test(wiggle_max_length_greedy);
}
