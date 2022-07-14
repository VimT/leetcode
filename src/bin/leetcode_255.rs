//! 验证前序遍历序列二叉搜索树

/// 递减单调栈，到右子树的时候，会把平级的左子树出栈，新元素必大于出栈元素
pub fn verify_preorder(preorder: Vec<i32>) -> bool {
    let mut s = vec![];
    let mut tmp = i32::MIN;
    for num in preorder {
        if num < tmp { return false; }
        while !s.is_empty() && *s.last().unwrap() < num {
            tmp = s.pop().unwrap();
        }
        s.push(num);
    }
    true
}

pub fn verify_preorder_dfs(preorder: Vec<i32>) -> bool {
    fn dfs(preorder: &[i32]) -> (i32, i32, bool) {
        if preorder.is_empty() { return (0, 0, true); }
        let mid = preorder[0];
        let mut i = 1;
        while i < preorder.len() && preorder[i] < mid {
            i += 1;
        }
        let mut min = mid;
        let mut max = mid;
        let mut ok = true;
        if i > 1 {
            let (lmin, lmax, lok) = dfs(&preorder[1..i]);
            min = min.min(lmin);
            if !lok || lmax >= mid { ok = false }
        }
        if i < preorder.len() {
            let (rmin, rmax, rok) = dfs(&preorder[i..]);
            max = max.max(rmax);
            if !rok || rmin <= mid { ok = false; }
        }
        (min, max, ok)
    }
    dfs(&preorder).2
}

fn main() {
    fn test(func: fn(preorder: Vec<i32>) -> bool) {
        assert_eq!(func(vec![5, 2, 1, 3, 6]), true);
        assert_eq!(func(vec![5, 2, 6, 1, 3]), false);
    }
    test(verify_preorder);
    test(verify_preorder_dfs);
}
