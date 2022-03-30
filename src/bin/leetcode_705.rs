struct MyHashSet {
    buckets: [i128; 8000],
}


impl MyHashSet {
    fn new() -> Self {
        return MyHashSet { buckets: [0; 8000] };
    }

    fn add(&mut self, key: i32) {
        let offset = (key >> 7) as usize;
        let bit_offset = key & 127;
        if self.buckets[offset] >> bit_offset & 1 == 0 {
            self.buckets[offset] |= 1 << bit_offset;
        }
    }

    fn remove(&mut self, key: i32) {
        let offset = (key >> 7) as usize;
        let bit_offset = key & 127;
        self.buckets[offset] &= !(1 << bit_offset);
    }

    fn contains(&self, key: i32) -> bool {
        let offset = (key >> 7) as usize;
        let bit_offset = key & 127;
        self.buckets[offset] >> bit_offset & 1 == 1
    }
}

fn main() {
    let mut hash = MyHashSet::new();
    hash.add(88);
    println!("{}", hash.contains(88));
    hash.remove(88);
    println!("{}", hash.contains(88));
}
