//! 统计对称整数的数目

pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    unsafe {
        const MAX: usize = 10001;
        static mut TABLE: [i32; MAX] = [0; MAX];
        static mut INIT: bool = false;
        if !INIT {
            let mut cur = 0;
            for num in 1..MAX {
                let s = num.to_string().into_bytes();
                if s.len() & 1 == 0 && s[..s.len() / 2].iter().zip(&s[s.len() / 2..]).map(|(&a, &b)| a as i8 - b as i8).sum::<i8>() == 0 {
                    cur += 1;
                }
                TABLE[num] = cur;
            }
            INIT = true;
        }
        TABLE[high as usize] - TABLE[low as usize - 1]
    }
}

fn main() {
    fn test(func: fn(low: i32, high: i32) -> i32) {
        assert_eq!(func(1, 100), 9);
        assert_eq!(func(1200, 1230), 4);
    }
    test(count_symmetric_integers);
}
