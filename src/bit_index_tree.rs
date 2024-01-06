pub trait BitVal {
    type ValType: Clone;
    fn identity() -> Self::ValType;
    fn op(f: &Self::ValType, g: &Self::ValType) -> Self::ValType;
}

pub struct BITree<T: BitVal> {
    data: Vec<T::ValType>,
}

impl<T: BitVal> BITree<T> {
    pub fn new(m: usize) -> Self {
        Self { data: vec![T::identity(); m + 1] }
    }
    pub fn update(&mut self, mut ind: usize, val: &T::ValType) {
        ind += 1;
        while ind < self.data.len() {
            self.data[ind] = T::op(&self.data[ind], val);
            ind += ind & ind.wrapping_neg();
        }
    }
    /// 查询 [0, ind) 的区间值
    pub fn query_prefix(&self, mut ind: usize) -> T::ValType {
        assert!(ind < self.data.len());
        let mut val = T::identity();
        while ind > 0 {
            val = T::op(&val, &self.data[ind]);
            ind &= ind - 1;
        }
        val
    }
}