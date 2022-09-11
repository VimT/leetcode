//! 删除操作后的最大子段和

/// 并查集，倒序
pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
    let len = nums.len();
    let mut fa = (0..len + 1).collect();
    let mut sum = vec![0; len + 1];
    fn find(fa: &mut Vec<usize>, x: usize) -> usize {
        return if fa[x] == x {
            x
        } else {
            fa[x] = find(fa, fa[x]);
            fa[x]
        };
    }
    let mut result = vec![0; len];
    for i in (1..len).rev() {
        let x = remove_queries[i] as usize;
        let to = find(&mut fa, x + 1);
        fa[x] = to; // 合并x和x+1
        sum[to] += sum[x] + nums[x] as i64;
        result[i - 1] = result[i].max(sum[to]);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64>) {
        assert_eq!(func(vec![1, 2, 5, 6, 1], vec![0, 3, 2, 4, 1]), vec![14, 7, 2, 2, 0]);
        assert_eq!(func(vec![3, 2, 11, 1], vec![3, 2, 1, 0]), vec![16, 5, 3, 0]);
    }
    test(maximum_segment_sum);
}
