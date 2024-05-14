//! 找出强数对的最大异或值 II

use std::collections::HashMap;

/// 0-1 Trie
pub fn maximum_strong_pair_xor(mut nums: Vec<i32>) -> i32 {
    const MAX_BIT: usize = 19;
    #[derive(Debug, Default)]
    struct Trie {
        children: [Option<Box<Trie>>; 2],
        cnt: i32, // 子树大小
    }
    impl Trie {
        fn insert(&mut self, val: usize) {
            let mut cur = self;
            for i in (0..=MAX_BIT).rev() {
                let bit = val >> i & 1;
                let child = cur.children[bit].get_or_insert_with(Default::default);
                child.cnt += 1;
                cur = child;
            }
        }

        fn remove(&mut self, val: usize) {
            let mut cur = self;
            for i in (0..=MAX_BIT).rev() {
                let bit = val >> i & 1;
                let child = cur.children[bit].get_or_insert_with(Default::default);
                child.cnt -= 1;
                cur = child;
            }
        }

        fn max_xor(&self, val: usize) -> i32 {
            let mut cur = self;
            let mut result = 0;
            for i in (0..=MAX_BIT).rev() {
                let mut bit = val >> i & 1;
                let child = cur.children[1 - bit].as_ref();
                if let Some(child) = child {
                    if child.cnt > 0 {
                        result |= 1 << i;
                        bit = 1 - bit;
                    }
                }
                cur = cur.children[bit].as_ref().unwrap();
            }
            result
        }
    }

    nums.sort_unstable();
    let mut t = Trie::default();
    let mut result = 0;
    let mut left = 0;
    for &y in &nums {
        t.insert(y as usize);
        while nums[left] * 2 < y {
            t.remove(nums[left] as usize);
            left += 1;
        }
        result = result.max(t.max_xor(y as usize));
    }
    result
}

/// hashmap: 一边遍历数组，一边记录每个 key 对应的最大的 nums[i]
pub fn maximum_strong_pair_xor2(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut result = 0;
    let mut mask = 0;
    let len = nums.len();
    let high_bit = 32 - nums[len - 1].leading_zeros();
    for i in (0..high_bit).rev() {
        mask |= 1 << i;
        let new_result = result | 1 << i; // 这个比特位可以是1吗？
        let mut d = HashMap::new();
        for &y in &nums {
            let mask_y = y & mask; // 低于i的比特为设置位0
            if let Some(&x) = d.get(&(new_result ^ mask_y)) {
                if x * 2 >= y {
                    result = new_result;
                    break;
                }
            }
            d.insert(mask_y, y);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 7);
        assert_eq!(func(vec![10, 100]), 0);
        assert_eq!(func(vec![500, 520, 2500, 3000]), 1020);
    }
    test(maximum_strong_pair_xor);
    test(maximum_strong_pair_xor2);
}
