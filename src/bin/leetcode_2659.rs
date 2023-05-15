//! 将数组清空

struct BIT {
    tree: Vec<i32>,
}

impl BIT {
    fn update(&mut self, mut x: usize) {
        while x < self.tree.len() {
            self.tree[x] += 1;
            x += lowbit(x);
        }
    }
    fn query(&mut self, mut x: usize) -> usize {
        let mut result = 0;
        while x > 0 {
            result += self.tree[x];
            x -= lowbit(x);
        }
        result as usize
    }
}

fn lowbit(x: usize) -> usize {
    let x = x as i32;
    (x & -x) as usize
}

/// 树状数组维护中间被删除的个数
pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
    let mut nums: Vec<(i32, usize)> = nums.into_iter().zip(0..).collect();
    let len = nums.len();
    let mut bit = BIT { tree: vec![0; len + 1] };
    nums.sort_unstable();
    let mut cur = 0;
    let mut result = 0;
    for (_, pos) in nums {
        if pos >= cur {
            result += pos + bit.query(cur) - cur - bit.query(pos + 1) + 1;
            cur = pos;
        } else {
            let right = len + bit.query(cur + 1) - 1 - cur - bit.query(len);
            let left = pos - bit.query(pos + 1);
            result += left + right + 1;
            cur = pos;
        }
        bit.update(pos + 1);
    }
    result as i64
}

/// 脑筋急转弯
pub fn count_operations_to_empty_array2(nums: Vec<i32>) -> i64 {
    let mut nums: Vec<(i32, usize)> = nums.into_iter().zip(0..).collect();
    let len = nums.len();
    nums.sort_unstable();
    let mut result = len;
    for i in 1..len {
        if nums[i].1 < nums[i - 1].1 {
            result += len - i;
        }
    }
    result as i64
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![3, 4, -1]), 5);
        assert_eq!(func(vec![1, 2, 4, 3]), 5);
        assert_eq!(func(vec![1, 2, 3]), 3);
    }
    test(count_operations_to_empty_array);
    test(count_operations_to_empty_array2);
}
