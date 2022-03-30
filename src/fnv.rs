use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

pub struct FnvHasher(u64);

impl Default for FnvHasher {
    #[inline]
    fn default() -> FnvHasher {
        FnvHasher(0xcbf29ce484222325)
    }
}

impl FnvHasher {
    #[inline]
    pub fn with_key(key: u64) -> FnvHasher {
        FnvHasher(key)
    }
}

impl Hasher for FnvHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;

        for byte in bytes.iter() {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        *self = FnvHasher(hash);
    }
}

pub type FnvBuildHasher = BuildHasherDefault<FnvHasher>;
pub type FnvHashMap<K, V> = HashMap<K, V, FnvBuildHasher>;
pub type FnvHashSet<T> = HashSet<T, FnvBuildHasher>;