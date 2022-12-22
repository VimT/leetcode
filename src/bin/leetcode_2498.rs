//! 青蛙过河 II

/// 脑筋急转弯。。。
pub fn max_jump(stones: Vec<i32>) -> i32 {
    let mut result = stones[1] - stones[0];
    for i in 0..stones.len() - 2 {
        result = result.max(stones[i + 2] - stones[i]);
    }
    result
}

/// 二分，找最远跳过去，再看能不能跳回来
pub fn max_jump2(stones: Vec<i32>) -> i32 {
    let len = stones.len();
    let mut left = 1;
    let mut right = stones[len - 1];
    let mut seen = vec![false; len];
    while left < right {
        seen.fill(false);
        let mid = (left + right) / 2;
        let mut i = 0;
        let mut ok = true;
        while i < len - 1 {
            let ni = stones.binary_search(&(stones[i] + mid)).unwrap_or_else(|x| x - 1);
            seen[ni] = true;
            if ni < len - 1 && (ni - i) <= 1 {
                ok = false;
                break;
            }
            i = ni
        }
        if ok {
            // 跳回来
            while i > 0 {
                let mut ni = i - 1;
                while ni > 0 && seen[ni] {
                    ni -= 1;
                }
                if stones[i] - stones[ni] > mid {
                    ok = false;
                    break;
                }
                i = ni;
            }
        }
        if ok {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    fn test(func: fn(stones: Vec<i32>) -> i32) {
        assert_eq!(func(vec![0, 5, 12, 25, 28, 35]), 20);
        assert_eq!(func(vec![0, 2, 5, 6, 7]), 5);
        assert_eq!(func(vec![0, 3, 9]), 9);
    }
    test(max_jump);
    test(max_jump2);
}
