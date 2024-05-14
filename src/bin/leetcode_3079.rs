//! 求出加密整数的和

pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    nums.into_iter().map(|x| {
        let s = x.to_string();
        let &max_num = s.as_bytes().iter().max().unwrap();
        unsafe { String::from_utf8_unchecked(vec![max_num; s.len()]) }.parse::<i32>().unwrap()
    }).sum()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3]), 6);
        assert_eq!(func(vec![10, 21, 31]), 66);
    }
    test(sum_of_encrypted_int);
}
