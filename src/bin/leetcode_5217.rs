//! 将杂乱无章的数字排序

pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_by_key(|x| {
        if *x == 0 { return mapping[0]; }
        let mut v = vec![];
        let mut num = *x;
        while num > 0 {
            v.push(mapping[(num % 10) as usize]);
            num /= 10;
        }
        v.reverse();
        for wei in v {
            num = num * 10 + wei;
        }
        num
    });
    nums
}

fn main() {
    assert_eq!(sort_jumbled(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    assert_eq!(sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]), [338, 38, 991]);
    assert_eq!(sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]), [123, 456, 789]);
}
