//! 天际线问题

use std::collections::BinaryHeap;

/// 最大堆法，存储当前最大高度
pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut ans = vec![vec![0, 0]];
    let mut points = vec![];
    heap.push((0, i32::MAX));
    // 这里先push 一个负值，排序后，相邻节点，左节点会优先遍历，保持了最大高度
    for i in buildings {
        points.push((i[0], -i[2], i[1]));
        points.push((i[1], i[2], 0));
    }
    points.sort();

    for (l, h, r) in points {
        // 保证当前堆顶为去除之前建筑物 右端点的最大值
        while heap.len() > 1 && heap.peek().unwrap().1 <= l {
            heap.pop();
        }
        if h < 0 {
            heap.push((-h, r));
        }
        let current_max_h = heap.peek().unwrap().0;
        if ans.last().unwrap()[1] != current_max_h {
            ans.push(vec![l, current_max_h])
        }
    }
    ans.remove(0);
    ans
}


fn main() {
    assert_eq!(get_skyline(vec![vec![2, 9, 10], vec![3, 7, 15], vec![5, 12, 12], vec![15, 20, 10], vec![19, 24, 8]]), vec![vec![2, 10], vec![3, 15], vec![7, 12], vec![12, 0], vec![15, 10], vec![20, 8], vec![24, 0]]);
    assert_eq!(get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]), vec![vec![0, 3], vec![5, 0]]);
}
