//! 不包含相邻元素的子序列的最大和

use leetcode::segment_tree::{SimpleSegmentSpec, SimpleSegmentTree};

pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let len = nums.len();
    enum Spec {}
    impl SimpleSegmentSpec for Spec {
        type ValueType = Value;

        fn identify() -> Self::ValueType {
            Value::default()
        }

        fn combine(a: &Self::ValueType, b: &Self::ValueType) -> Self::ValueType {
            Value {
                f11: (a.f10 + b.f11).max(a.f11 + b.f01),
                f00: (a.f01 + b.f00).max(a.f00 + b.f10),
                f10: (a.f10 + b.f10).max(a.f11 + b.f00),
                f01: (a.f01 + b.f01).max(a.f00 + b.f11),
            }
        }
    }
    #[derive(Clone, Debug, Default)]
    struct Value {
        f11: i32,
        f00: i32,
        f10: i32,
        f01: i32,
    }
    impl Value {
        fn new(val: i32) -> Self { return Value { f11: val.max(0), f00: 0, f10: 0, f01: 0 }; }
        fn max(&self) -> i32 { return self.f11.max(self.f00).max(self.f10).max(self.f01); }
    }

    let val: Vec<Value> = nums.into_iter().map(Value::new).collect();
    let mut tree = SimpleSegmentTree::<Spec>::new(&val);
    let mut result = 0;
    const MOD: i32 = 1e9 as i32 + 7;
    for query in queries {
        let (pos, x) = (query[0] as usize, query[1]);
        tree.update(pos, Value::new(x));
        let value = tree.query(0, len - 1);
        result = (result + value.max()) % MOD;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![2, 3, 1, -1, 1, -2], vec![vec![0, -2], vec![0, 0]]), 8);
        assert_eq!(func(vec![1, -1, 2, 0, -1, -2, -2, -2], vec![vec![6, 2], vec![0, -2], vec![2, 0]]), 11);
        assert_eq!(func(vec![0, 3, 3, 3, 1, -2], vec![vec![4, 0], vec![1, 0]]), 9);
        assert_eq!(func(vec![3, 5, 9], vec![vec![1, -2], vec![0, -3]]), 21);
        assert_eq!(func(vec![0, -1], vec![vec![0, -5]]), 0);
    }
    test(maximum_sum_subsequence);
}
