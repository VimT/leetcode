//! 求出 MK 平均值

use std::collections::{BTreeMap, VecDeque};

trait MultiSet<T> {
    fn first(&self) -> Option<&T>;
    fn last(&self) -> Option<&T>;
    fn pop_first0(&mut self) -> Option<T>;
    fn pop_last0(&mut self) -> Option<T>;
    fn insert0(&mut self, val: T);
    fn remove0(&mut self, val: T);
}

impl<T: Clone + Ord> MultiSet<T> for BTreeMap<T, i32> {
    fn first(&self) -> Option<&T> {
        self.range(..).next().map(|x| x.0)
    }

    fn last(&self) -> Option<&T> {
        self.range(..).last().map(|x| x.0)
    }

    fn pop_first0(&mut self) -> Option<T> {
        self.first().cloned().map(|k| {
            self.remove0(k.clone());
            k
        })
    }

    fn pop_last0(&mut self) -> Option<T> {
        self.last().cloned().map(|k| {
            self.remove0(k.clone());
            k
        })
    }

    fn insert0(&mut self, val: T) {
        *self.entry(val).or_default() += 1;
    }

    fn remove0(&mut self, val: T) {
        if let std::collections::btree_map::Entry::Occupied(mut v) = self.entry(val) {
            *v.get_mut() -= 1;
            if *v.get() == 0 {
                v.remove();
            }
        }
    }
}

type BtreeMultiSet<T> = BTreeMap<T, i32>;

#[derive(Default)]
struct MKAverage {
    s1: BtreeMultiSet<i32>,
    s2: BtreeMultiSet<i32>,
    s3: BtreeMultiSet<i32>,
    q: VecDeque<i32>,
    k: usize,
    m: usize,
    sum: i64,
}


impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        Self {
            k: k as usize,
            m: m as usize,
            ..Default::default()
        }
    }

    fn add_element(&mut self, num: i32) {
        self.q.push_back(num);
        if self.q.len() <= self.m {
            self.s2.insert0(num);
            self.sum += num as i64 as i64;
            if self.q.len() == self.m {
                for _ in 0..self.k {
                    let num = self.s2.pop_first0().unwrap();
                    self.sum -= num as i64;
                    self.s1.insert0(num);
                }
                for _ in 0..self.k {
                    let num = self.s2.pop_last0().unwrap();
                    self.sum -= num as i64;
                    self.s3.insert0(num);
                }
            }
        } else if self.q.len() > self.m {
            let mut one = *self.s2.first().unwrap();
            let mut two = *self.s2.last().unwrap();
            if num < one {
                self.s1.insert0(num);
                let num = self.s1.pop_last0().unwrap();
                self.s2.insert0(num);
                self.sum += num as i64;
                one = num;
            } else if num > two {
                self.s3.insert0(num);
                let num = self.s3.pop_first0().unwrap();
                self.s2.insert0(num);
                self.sum += num as i64;
                two = num;
            } else {
                self.s2.insert0(num);
                self.sum += num as i64;
            }

            let pop = self.q.pop_front().unwrap();
            if pop < one {
                self.s1.remove0(pop);
                self.s1.insert0(one);
                self.s2.remove0(one);
                self.sum -= one as i64;
            } else if pop > two {
                self.s3.remove0(pop);
                self.s3.insert0(two);
                self.s2.remove0(two);
                self.sum -= two as i64;
            } else {
                self.s2.remove0(pop);
                self.sum -= pop as i64;
            }
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.q.len() < self.m {
            return -1;
        }
        (self.sum / (self.m - 2 * self.k) as i64) as i32
    }
}

fn main() {
    let mut obj = MKAverage::new(3, 1);
    obj.add_element(3);        // 当前元素为 [3]
    obj.add_element(1);        // 当前元素为 [3,1]
    assert_eq!(obj.calculate_mk_average(), -1); // 返回 -1 ，因为 m = 3 ，但数据流中只有 2 个元素
    obj.add_element(10);       // 当前元素为 [3,1,10]
    assert_eq!(obj.calculate_mk_average(), 3); // 最后 3 个元素为 [3,1,10]
    // 删除最小以及最大的 1 个元素后，容器为 [3]
    // [3] 的平均值等于 3/1 = 3 ，故返回 3
    obj.add_element(5);        // 当前元素为 [3,1,10,5]
    obj.add_element(5);        // 当前元素为 [3,1,10,5,5]
    obj.add_element(5);        // 当前元素为 [3,1,10,5,5,5]
    assert_eq!(obj.calculate_mk_average(), 5); // 最后 3 个元素为 [5,5,5]
    // 删除最小以及最大的 1 个元素后，容器为 [5]
    // [5] 的平均值等于 5/1 = 5 ，故返回 5
}
