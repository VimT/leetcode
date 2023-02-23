//! 更新数组后处理求和查询

pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let len = nums1.len();
    let mut tree = SegmentTree::new(&nums1);
    let mut cur_sum: i64 = nums2.into_iter().map(|x| x as i64).sum();
    let mut result = vec![];
    for query in queries {
        match query[0] {
            1 => tree.update(query[1] as usize, query[2] as usize),
            2 => cur_sum += tree.query(0, len - 1) as i64 * query[1] as i64,
            3 => result.push(cur_sum),
            _ => unreachable!()
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64>) {
        assert_eq!(func(vec![0,1,0,1,0,1,0,0,0,1],
                        vec![35,34,38,28,38,20,18,12,2,30],
                        vec![vec![1,2,4],vec![1,1,9],vec![3,0,0]]), vec![255]);
        assert_eq!(func(vec![1, 0, 1], vec![0, 0, 0], vec![vec![1, 1, 1], vec![2, 1, 0], vec![3, 0, 0]]), vec![3]);
        assert_eq!(func(vec![1], vec![5], vec![vec![2, 0, 0], vec![3, 0, 0]]), vec![5]);
    }
    test(handle_query);
}
