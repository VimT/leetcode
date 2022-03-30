//! 最大频率栈


use std::collections::HashMap;

/** 原本的实现是，其实不用这么麻烦，push(5) 重复两次，频率1－5不用删。。保留，新增一个2－5就好了。。
#[derive(Default)]
struct FreqStack {
    cnt_value: BTreeMap<i32, BTreeSet<(i32, i32)>>,
    value_cnt: HashMap<i32, Vec<i32>>,
    offset: i32,
}
 **/
#[derive(Default)]
struct FreqStack {
    freq_value: HashMap<i32, Vec<i32>>,
    value_freq: HashMap<i32, i32>,
    max_freq: i32,
}


impl FreqStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        let cur_freq = self.value_freq.entry(val).or_default();
        *cur_freq += 1;
        self.max_freq = self.max_freq.max(*cur_freq);
        self.freq_value.entry(*cur_freq).or_default().push(val);
    }

    fn pop(&mut self) -> i32 {
        let list = self.freq_value.get_mut(&self.max_freq).unwrap();
        let val = list.pop().unwrap();
        if list.is_empty() {
            self.max_freq -= 1;
        }
        *self.value_freq.get_mut(&val).unwrap() -= 1;
        val
    }
}


fn main() {
    let mut fs = FreqStack::new();
    fs.push(5);//堆栈为 [5]
    fs.push(7);//堆栈是 [5,7]
    fs.push(5);//堆栈是 [5,7,5]
    fs.push(7);//堆栈是 [5,7,5,7]
    fs.push(4);//堆栈是 [5,7,5,7,4]
    fs.push(5);//堆栈是 [5,7,5,7,4,5]
    assert_eq!(fs.pop(), 5);//返回 5 ，因为 5 出现频率最高。堆栈变成 [5,7,5,7,4]。
    assert_eq!(fs.pop(), 7);//返回 7 ，因为 5 和 7 出现频率最高，但7最接近顶部。堆栈变成 [5,7,5,4]。
    assert_eq!(fs.pop(), 5);//返回 5 ，因为 5 出现频率最高。堆栈变成 [5,7,4]。
    assert_eq!(fs.pop(), 4);//返回 4 ，因为 4, 5 和 7 出现频率最高，但 4 是最接近顶部的。堆栈变成 [5,7]。
}