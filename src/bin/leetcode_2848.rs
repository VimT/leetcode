//! 与车相交的点

pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
    let max = nums.iter().map(|x| x[1]).max().unwrap() as usize;
    let mut points = vec![0; max + 2];
    for num in nums {
        points[num[0] as usize] += 1;
        points[num[1] as usize + 1] -= 1;
    }
    points[1..=max].iter().scan(0, |sum, &num| {
        *sum += num;
        Some((*sum > 0) as i32)
    }).sum()
}

fn main() {
    fn test(func: fn(nums: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![3, 6], vec![1, 5], vec![4, 7]]), 7);
        assert_eq!(func(vec![vec![1, 3], vec![5, 8]]), 7);
    }
    test(number_of_points);
}
