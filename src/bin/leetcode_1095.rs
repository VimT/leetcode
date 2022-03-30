//! 山脉数组中查找目标值

use std::cell::RefCell;

pub struct MountainArray {
    arr: Vec<i32>,
    cnt: RefCell<u8>,
}

impl MountainArray {
    fn new(arr: Vec<i32>) -> Self {
        Self { arr, cnt: RefCell::new(0) }
    }
    fn get(&self, index: i32) -> i32 {
        if *self.cnt.borrow() > 100 { panic!() }
        *self.cnt.borrow_mut() += 1;
        return self.arr[index as usize];
    }
    fn length(&self) -> i32 {
        return self.arr.len() as i32;
    }
}

pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
    let mut left = 0;
    let len = mountain_arr.length();
    let mut right = len;
    if len == 0 { return -1; }
    while left < right {
        let mid = (left + right) / 2;
        let a = mountain_arr.get(mid);
        if mid + 1 == len {
            break;
        }
        let b = mountain_arr.get(mid + 1);
        if a < b {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    let mid = left;
    left = 0;
    right = mid;
    while left < right {
        let mid = (left + right) / 2;
        let val = mountain_arr.get(mid);
        if val < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if left < len && mountain_arr.get(left) == target {
        return left;
    }
    left = mid;
    right = len;
    while left < right {
        let mid = (left + right) / 2;
        let val = mountain_arr.get(mid);
        if val > target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if left < len && mountain_arr.get(left) == target {
        return left;
    }
    -1
}

fn main() {
    fn test(func: fn(target: i32, mountain_arr: &MountainArray) -> i32) {
        assert_eq!(func(0, &MountainArray::new(vec![1, 5, 2])), -1);
        assert_eq!(func(3, &MountainArray::new(vec![1, 2, 3, 4, 5, 3, 1])), 2);
        assert_eq!(func(3, &MountainArray::new(vec![0, 1, 2, 4, 2, 1])), -1);
    }
    test(find_in_mountain_array);
}
