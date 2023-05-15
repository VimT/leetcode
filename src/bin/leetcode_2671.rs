//! 频率跟踪器


use std::ops;

struct FastVec<T>(Vec<T>);
impl<T> ops::Index<usize> for FastVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.0.get_unchecked(index) }
    }
}
impl<T> ops::IndexMut<usize> for FastVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.0.get_unchecked_mut(index) }
    }
}

/// 这个题用 vec 居然比用map慢？？ 不科学
struct FrequencyTracker {
    num_freq: FastVec<usize>,
    freq_cnt: FastVec<i32>,
}


impl FrequencyTracker {
    fn new() -> Self {
        Self { num_freq: FastVec(vec![0; 100001]), freq_cnt: FastVec(vec![0; 200002]) }
    }

    fn add(&mut self, number: i32) {
        self.freq_cnt[self.num_freq[number as usize]] -= 1;
        self.num_freq[number as usize] += 1;
        self.freq_cnt[self.num_freq[number as usize]] += 1;
    }

    fn delete_one(&mut self, number: i32) {
        if self.num_freq[number as usize] == 0 {return}
        self.freq_cnt[self.num_freq[number as usize]] -= 1;
        self.num_freq[number as usize] -= 1;
        self.freq_cnt[self.num_freq[number as usize]] += 1;
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.freq_cnt[frequency as usize] > 0
    }
}


fn main() {
    let mut ft = FrequencyTracker::new();
    ft.add(1); // 数据结构现在包含 [1]
    ft.delete_one(1); // 数据结构现在为空 []
    assert_eq!(ft.has_frequency(1), false); // 返回 false ，因为数据结构为空
}
