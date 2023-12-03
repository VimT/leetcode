//! 找出满足差值条件的下标 II

pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    let len = nums.len();
    let mut mx = nums[0];
    let mut mn = nums[0];
    let mut mxi = 0;
    let mut mni = 0;
    let x = index_difference as usize;
    if x >= len { return vec![-1, -1]; }
    for i in 0..len - x {
        if nums[i] > mx {
            mx = nums[i];
            mxi = i;
        }
        if nums[i] < mn {
            mn = nums[i];
            mni = i;
        }
        if (nums[i + x] - mx).abs() >= value_difference {
            return vec![mxi as i32, (i + x) as i32];
        }
        if (nums[i + x] - mn).abs() >= value_difference {
            return vec![mni as i32, (i + x) as i32];
        }
    }
    vec![-1, -1]
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32>) {
        assert_eq!(func(vec![5, 1, 4, 1], 2, 4), vec![0, 3]);
        assert_eq!(func(vec![2, 1], 0, 0), vec![0, 0]);
        assert_eq!(func(vec![1, 2, 3], 2, 4), vec![-1, -1]);
    }
    test(find_indices);
}
