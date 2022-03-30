//! 存在重复元素 III

use std::ops::Bound::Included;

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let mut ni = nums.iter().enumerate().map(|(i, x)| (*x as i64, i)).collect::<Vec<(i64, usize)>>();
    ni.sort_unstable();
    let t = t as i64;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if ni[j].0 - ni[i].0 > t {
                break;
            }
            if (ni[i].1 as i32 - ni[j].1 as i32).abs() <= k {
                return true;
            }
        }
    }
    false
}

// O(nlog(min(n,k)))
// ! use avl tree instead of vec, for vec insert has O(n)
pub fn contains_nearby_almost_duplicate_avl(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let mut set = std::collections::BTreeSet::new();
    let k = k as usize;
    let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<i64>>();
    let t = t as i64;
    for i in 0..nums.len() {
        for _ in set.range((Included(&nums[i] - t), Included(&nums[i] + t))) {
            return true;
        }
        set.insert(nums[i]);
        if set.len() > k {
            set.remove(&(nums[i - k]));
        }
    }
    false
}

pub fn contains_nearby_almost_duplicate_bucket(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let mut d: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
    let k = k as usize;
    let t = t as i64;
    let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<i64>>();
    let get_id = |x: i64, w: i64| -> i64 {
        return if x < 0 {
            x / w - 1
        } else {
            x / w
        };
    };
    let w = t + 1;
    for i in 0..nums.len() {
        let id = get_id(nums[i], w);
        if d.contains_key(&id) {
            return true;
        }
        if let Some(&v) = d.get(&(id - 1)) {
            if (nums[i] - v).abs() < w {
                return true;
            }
        }
        if let Some(&v) = d.get(&(id + 1)) {
            if (nums[i] - v).abs() < w {
                return true;
            }
        }
        d.insert(id, nums[i]);
        if i >= k {
            d.remove(&get_id(nums[i - k], w));
        }
    }
    false
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32, t: i32) -> bool) {
        assert_eq!(func(vec![1, 2, 3, 1], 3, 0), true);
        assert_eq!(func(vec![1, 0, 1, 1], 1, 2), true);
        assert_eq!(func(vec![1, 5, 9, 1, 5, 9], 2, 3), false);
    }
    test(contains_nearby_almost_duplicate);
    test(contains_nearby_almost_duplicate_avl);
    test(contains_nearby_almost_duplicate_bucket);
}
