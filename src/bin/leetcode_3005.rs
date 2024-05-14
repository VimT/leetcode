//! 最大频率元素计数

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut cnt = vec![0; 101];
    for num in nums {
        cnt[num as usize] += 1;
    }
    cnt.sort_unstable();
    let last = cnt.pop().unwrap();
    let mut result = last;
    while *cnt.last().unwrap() == last {
        result += cnt.pop().unwrap();
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 2, 3, 1, 4]), 4);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 5);
    }
    test(max_frequency_elements);
}
