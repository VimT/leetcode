//! 按公因数计算最大组件大小


use std::collections::HashMap;
use leetcode::union_find::UnionFind;


static mut ZHISHU: [usize; 5133] = [0; 5133];

pub fn largest_component_size(nums: Vec<i32>) -> i32 {
    unsafe {
        if ZHISHU[0] == 0 {
            let mut idx = 0;
            for i in 2..50000_usize {
                let mut is = true;
                for j in 2..=(i as f64).sqrt() as usize {
                    if i % j == 0 {
                        is = false;
                        break;
                    }
                }
                if is {
                    ZHISHU[idx] = i;
                    idx += 1;
                }
            }
        }
        let max = *nums.iter().max().unwrap() as usize;
        let mut uf = UnionFind::new(max + 1);
        let mut m = vec![false; max + 1];
        for &i in &nums {
            m[i as usize] = true;
        }

        for i in 0..5133 {
            let mut v = vec![];
            for j in 1..=max / ZHISHU[i] {
                if m[ZHISHU[i] * j] {
                    v.push(ZHISHU[i] * j);
                }
            }
            if v.len() > 1 {
                for j in 1..v.len() {
                    us.union(v[j] as usize, v[j - 1] as usize);
                }
            }
        }
        let mut result = HashMap::new();
        for i in nums {
            *result.entry(us.find(i as usize)).or_default() += 1;
        }
        *result.values().max().unwrap()
    }
}

fn main() {
    assert_eq!(largest_component_size(vec![4, 6, 15, 35]), 4);
    assert_eq!(largest_component_size(vec![20, 50, 9, 63]), 2);
    assert_eq!(largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39]), 8);
}
