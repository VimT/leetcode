//! 可变区间和：树状数组

struct NumArray {
    t: Vec<i32>,
}

#[inline]
fn lowbit(x: i32) -> i32 {
    return x & (-x);
}


impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        let len = nums.len();
        nums.insert(0, 0);
        for i in 1..=len {
            let j = i + lowbit(i as i32) as usize;
            if j <= len {
                nums[j] += nums[i];
            }
        }
        NumArray { t: nums }
    }

    fn update(&mut self, index: i32, val: i32) {
        let diff = val - self.sum_range(index, index);
        let mut index = index as usize + 1;
        while index < self.t.len() {
            self.t[index] += diff;
            index += lowbit(index as i32) as usize;
        }
    }

    fn sum(&self, mut index: usize) -> i32 {
        let mut ret = 0;
        while index > 0 {
            ret += self.t[index];
            index -= lowbit(index as i32) as usize;
        }
        ret
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum(right as usize + 1) - self.sum(left as usize)
    }
}

fn main() {
    let mut na = NumArray::new(vec![1, 3, 5]);
    assert_eq!(na.sum_range(0, 2), 9);
    na.update(1, 2);
    assert_eq!(na.sum_range(0, 2), 8);
}