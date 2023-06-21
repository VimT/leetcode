//! 找到最接近目标值的函数值

/// 与的性质，数字会越来越小。并且 任意区间 的与集结果不会超过 log(num) 个
pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
    let mut valid = vec![arr[0]];
    let mut result = (arr[0] - target).abs();
    for num in arr {
        let mut new_valid = vec![num];
        result = result.min((num - target).abs());
        for prev in valid {
            new_valid.push(prev & num);
            result = result.min(((prev & num) - target).abs());
        }
        new_valid.dedup();
        valid = new_valid;
    }
    result
}

/// 线段树+双指针，妙啊
pub fn closest_to_target2(arr: Vec<i32>, target: i32) -> i32 {
    fn build(i: i32, l: i32, r: i32, arr: &Vec<i32>, tree: &mut Vec<i32>) {
        if l == r {
            tree[i as usize] = if l < arr.len() as i32 { arr[l as usize] } else { 0 };
            return;
        }
        let mid = (r + l) / 2;
        build(i * 2, l, mid, arr, tree);
        build(i * 2 + 1, mid + 1, r, arr, tree);
        tree[i as usize] = tree[i as usize * 2] & tree[i as usize * 2 + 1];
    }
    fn ask(i: i32, l: i32, r: i32, ll: i32, rr: i32, tree: &Vec<i32>) -> i32 {
        if l == ll && rr == r {
            return tree[i as usize];
        }
        let mid = (r + l) / 2;
        if rr <= mid {
            ask(i * 2, l, mid, ll, rr, tree)
        } else if mid < ll {
            ask(i * 2 + 1, mid + 1, r, ll, rr, tree)
        } else {
            ask(i * 2, l, mid, ll, mid, tree) & ask(i * 2 + 1, mid + 1, r, mid + 1, rr, tree)
        }
    }
    let n = arr.len() as i32;
    let mut tree = vec![0; arr.len() * 4 + 1];
    build(1, 0, n - 1, &arr, &mut tree);
    let (mut l, mut r) = (0, 0);
    let mut ans = i32::MAX;
    while r < n {
        let t = ask(1, 0, n - 1, l, r, &tree);
        if t < target {
            l += 1;
            r = r.max(l);
        } else {
            r += 1;
        }
        ans = ans.min((t - target).abs());
    }
    ans
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![5, 89, 79, 44, 45, 79], 965), 876);
        assert_eq!(func(vec![9, 12, 3, 7, 15], 5), 2);
        assert_eq!(func(vec![1000000, 1000000, 1000000], 1), 999999);
        assert_eq!(func(vec![1, 2, 4, 8, 16], 0), 0);
    }
    test(closest_to_target);
    test(closest_to_target2);
}
