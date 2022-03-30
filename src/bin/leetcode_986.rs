//! 区间列表的交集

pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len1 = first_list.len();
    let len2 = second_list.len();
    let mut i = 0;
    let mut j = 0;
    let mut result = vec![];
    while i < len1 && j < len2 {
        let range1 = &first_list[i];
        let range2 = &second_list[j];
        if range1[1] < range2[0] {
            i += 1;
        } else if range2[1] < range1[0] {
            j += 1;
        } else if range1[1] < range2[1] {
            result.push(vec![range1[0].max(range2[0]), range1[1]]);
            i += 1;
        } else if range2[1] < range1[1] {
            result.push(vec![range1[0].max(range2[0]), range2[1]]);
            j += 1;
        } else {
            // range1[1] == range2[1]
            result.push(vec![range1[0].max(range2[0]), range1[1]]);
            i += 1;
            j += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(interval_intersection(vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]], vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]), vec![vec![1, 2], vec![5, 5], vec![8, 10], vec![15, 23], vec![24, 24], vec![25, 25]]);
    assert_eq!(interval_intersection(vec![vec![1, 3], vec![5, 9]], vec![]).is_empty(), true);
}
