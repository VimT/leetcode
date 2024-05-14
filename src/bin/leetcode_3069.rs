//! 将元素分配到两个数组中 I

pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut arr1 = vec![];
    let mut arr2 = vec![];
    arr1.push(nums[0]);
    arr2.push(nums[1]);
    let len = nums.len();
    for i in 2..len {
        if arr1.last().unwrap() > arr2.last().unwrap() {
            arr1.push(nums[i]);
        } else {
            arr2.push(nums[i]);
        }
    }
    arr1.extend_from_slice(&arr2);
    arr1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![2, 1, 3]), vec![2, 3, 1]);
        assert_eq!(func(vec![5, 4, 3, 8]), vec![5, 3, 4, 8]);
    }
    test(result_array);
}
