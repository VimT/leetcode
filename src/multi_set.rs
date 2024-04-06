pub use treap::TreapMultiSet;
pub use btree::BtreeMultiSet;

pub mod treap {
    use crate::treap::Treap;

    #[derive(Debug)]
    pub struct TreapMultiSet<K: Ord + Clone> {
        inner: Treap<K, usize>,
        size: usize,
    }

    impl<K: Ord + Clone> TreapMultiSet<K> {
        pub fn new() -> Self {
            Self { inner: Treap::new(), size: 0 }
        }
        pub fn insert(&mut self, key: K) {
            self.size += 1;
            if let Some(v) = self.inner.get_mut(&key) {
                *v += 1;
            } else {
                self.inner.insert(key.clone(), 1);
            }
        }
        pub fn remove(&mut self, key: &K) -> bool {
            if let Some(v) = self.inner.get_mut(key) {
                self.size -= 1;
                *v -= 1;
                if *v == 0 { self.inner.remove(key); }
                true
            } else {
                false
            }
        }
        pub fn len(&self) -> usize {
            self.size
        }
        pub fn first(&self) -> Option<&K> {
            self.inner.first().map(|(k, _)| k)
        }
        pub fn pop_first(&mut self) -> Option<K> {
            let mut result = None;
            let mut need_remove = false;
            if let Some((k, v)) = self.inner.first_mut() {
                self.size -= 1;
                *v -= 1;
                result = Some(k.clone());
                if *v == 0 { need_remove = true; }
            }
            if need_remove { self.inner.remove(&result.as_ref().unwrap()); }
            result
        }
        pub fn last(&self) -> Option<&K> {
            self.inner.last().map(|(k, _)| k)
        }
        pub fn pop_last(&mut self) -> Option<K> {
            let mut result = None;
            let mut need_remove = false;
            if let Some((k, v)) = self.inner.last_mut() {
                self.size -= 1;
                *v -= 1;
                result = Some(k.clone());
                if *v == 0 { need_remove = true; }
            }
            if need_remove { self.inner.remove(&result.as_ref().unwrap()); }
            result
        }
    }
}

pub mod btree {
    use std::collections::btree_map::Entry;
    use std::collections::BTreeMap;

    #[derive(Debug)]
    pub struct BtreeMultiSet<K: Ord + Clone> {
        inner: BTreeMap<K, usize>,
        size: usize,
    }

    impl<K: Ord + Clone> BtreeMultiSet<K> {
        pub fn new() -> Self {
            Self { inner: BTreeMap::new(), size: 0 }
        }
        pub fn insert(&mut self, key: K) {
            self.size += 1;
            match self.inner.entry(key) {
                Entry::Vacant(v) => { v.insert(1); }
                Entry::Occupied(mut v) => { *v.get_mut() += 1; }
            }
        }
        pub fn remove(&mut self, key: K) -> bool {
            if let Entry::Occupied(mut v) = self.inner.entry(key) {
                self.size -= 1;
                *v.get_mut() -= 1;
                if *v.get() == 0 { v.remove(); }
                true
            } else {
                false
            }
        }
        pub fn len(&self) -> usize {
            self.size
        }
        pub fn first(&self) -> Option<&K> {
            self.inner.first_key_value().map(|(k, _)| k)
        }
        pub fn pop_first(&mut self) -> Option<K> {
            let result = self.inner.first_entry().map(|mut e| {
                let result = e.key().clone();
                *e.get_mut() -= 1;
                if *e.get() == 0 { e.remove(); }
                result
            });
            if result.is_some() { self.size -= 1; }
            result
        }
        pub fn last(&self) -> Option<&K> {
            self.inner.last_key_value().map(|(k, _)| k)
        }
        pub fn pop_last(&mut self) -> Option<K> {
            let result = self.inner.last_entry().map(|mut e| {
                let result = e.key().clone();
                *e.get_mut() -= 1;
                if *e.get() == 0 { e.remove(); }
                result
            });
            if result.is_some() { self.size -= 1; }
            result
        }
    }
}