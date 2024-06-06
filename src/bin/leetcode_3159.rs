//! 查询数组中元素的出现位置

pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
    let m: Vec<i32> = nums.into_iter().enumerate().filter(|&(_, num)| num == x).map(|x| x.0 as i32).collect();
    queries.into_iter().map(|x| {
        if x > m.len() as i32 {
            -1
        } else {
            m[x as usize - 1]
        }
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32>) {
        assert_eq!(func(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1), vec![0, -1, 2, -1]);
        assert_eq!(func(vec![1, 2, 3], vec![10], 5), vec![-1]);
    }
    test(occurrences_of_element);
}
