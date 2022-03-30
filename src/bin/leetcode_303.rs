//! 区域和检索 - 数组不可变

struct NumArray {
    ps: Vec<i32>,
}


impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut ps = vec![0; nums.len() + 1];
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            ps[i + 1] = sum;
        }
        NumArray { ps }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        return self.ps[j as usize + 1] - self.ps[i as usize];
    }
}

fn main() {
    let na = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(na.sum_range(0, 2), 1);
    assert_eq!(na.sum_range(2, 5), -1);
    assert_eq!(na.sum_range(0, 5), -3);
}
