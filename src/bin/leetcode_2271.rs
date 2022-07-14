//! 毯子覆盖的最多白色砖块数

/// 二分
pub fn maximum_white_tiles(mut tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
    tiles.sort_unstable();
    let len = tiles.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + tiles[i][1] - tiles[i][0] + 1;
    }
    let mut result = 0;
    for i in 0..len {
        let end = tiles[i][0] + carpet_len - 1;
        // 要搜右边
        let idx = tiles.binary_search_by_key(&end, |x| x[1]).unwrap_or_else(|x| x);
        let cur = presum[idx] - presum[i] + if idx < len { (end - tiles[idx][0] + 1).max(0) } else { 0 };
        result = result.max(cur);
    }
    result
}

/// 双指针
pub fn maximum_white_tiles2(mut tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
    tiles.sort_unstable();
    let len = tiles.len();
    let mut left = 0;
    let mut right = 0;
    let mut sum = 0;
    let mut result = 0;
    while right < len {
        let start = tiles[left][0];
        let end = start + carpet_len - 1;
        if tiles[right][1] <= end {
            sum += tiles[right][1] - tiles[right][0] + 1;
            right += 1;
            result = result.max(sum);
        } else {
            if end >= tiles[right][0] {
                result = result.max(sum + end - tiles[right][0] + 1);
            }
            sum -= tiles[left][1] - tiles[left][0] + 1;
            left += 1;
        }
    }
    result
}


fn main() {
    fn test(func: fn(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32) {
        assert_eq!(func(vec![vec![1, 5], vec![10, 11], vec![12, 18], vec![20, 25], vec![30, 32]], 10), 9);
        assert_eq!(func(vec![vec![10, 11], vec![1, 1]], 2), 2);
    }
    test(maximum_white_tiles);
    test(maximum_white_tiles2);
}
