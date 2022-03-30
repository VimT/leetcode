//! 统计异或值在范围内的数对有多少


pub fn count_pairs_baoli(nums: Vec<i32>, low: i32, high: i32) -> i32 {
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        for j in i..len {
            let num = nums[i] ^ nums[j];
            if num >= low && num <= high {
                result += 1;
            }
        }
    }
    result
}

struct Trie {
    children: [Option<Box<Trie>>; 2],
    size: i32,
}

impl Trie {
    fn new() -> Self {
        Trie { children: [None, None], size: 0 }
    }

    fn insert(&mut self, x: i32) {
        let mut node = self;
        for i in (0..=15).rev() {
            let bit = x >> i & 1;
            if node.children[bit as usize].is_none() {
                node.children[bit as usize] = Some(Box::new(Trie::new()));
            }
            node = node.children[bit as usize].as_mut().unwrap();
            node.size += 1;
        }
    }

    fn query(&self, x: i32, high: i32) -> i32 {
        let mut node = self;
        let mut result = 0;
        for i in (0..=15).rev() {
            let high_bit = high >> i & 1;
            let x_bit = x >> i & 1;
            let next_node = if high_bit == 1 {
                if node.children[x_bit as usize].is_some() {
                    result += node.children[x_bit as usize].as_ref().unwrap().size;
                }
                &node.children[1 - x_bit as usize]
            } else {
                &node.children[x_bit as usize]
            };
            if next_node.is_none() {
                return result;
            }
            node = next_node.as_ref().unwrap();
        }
        result
    }
}


/// 也就是形如[a << K, (a + 1) << K)这样的前闭后开区间，
/// 它的特征在于任意一个数A与区间中每一个数异或得到的结果刚好也形成一个连续区间，
/// 而且这个连续区间也有相同的二进制前缀，
/// 实际上是[(a ^ (A >> K)) << K, ((a ^ (A >> K)) + 1) << K]，
/// 其中的原理不难明白：因为要异或的数前缀完全相同，所以结果的前缀部分也完全相同，而后面的部分包含了所有可能的二进制结果，异或是个一一映射，
/// 所以结果的后缀部分也包含了所有可能的二进制结果，
/// 容斥原理， [lo, hi] （闭区间，下同）内符合要求的个数等于 [0, hi] 内数目减去 [0, lo - 1] 内数目。
/// query(int x, int high)  查询之前插入过的数字跟数字x之间的异或值<high的个数
pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
    let mut trie = Trie::new();
    let mut result = 0;
    for i in nums {
        result += trie.query(i, high + 1) - trie.query(i, low);
        trie.insert(i);
    }
    result
}

fn main() {
    assert_eq!(count_pairs(vec![5202, 7809, 40, 6748, 7221, 4423, 2803, 4528, 2255, 2204, 6140, 3802, 4257, 2735, 6416, 75, 4432, 6641, 2595, 722, 4667, 2897, 4669, 6946, 2915, 3729, 4633, 1350, 8044, 8014, 2419, 4829, 6587, 5745, 6691, 7687, 406, 817, 6775, 682, 3753, 2477, 3534, 2910, 5753, 1984, 2571, 1663, 3993, 3419, 2130, 5496, 3868, 2927, 1044, 6919, 122, 4452, 5345, 3041, 703, 247, 2874, 6709, 1902, 1237, 2195, 6860, 6920, 5103, 2954, 5896, 145, 5323, 312, 6771, 1748, 348, 7798, 6800, 5300, 7014, 2773, 763, 1599, 1869, 5920, 1763, 5924, 1896, 1860, 4452, 6968, 6325, 1258, 7466, 2220, 6066, 7607, 1428, 6774, 7990, 4533, 2971, 5159, 6184, 2165, 370, 3033, 6007, 8062, 4713, 5465, 1498, 3346, 7562, 7044, 2921, 4195, 2975, 5901, 4574, 7874, 3184, 1292, 4661, 5634, 2134, 7578, 4817, 4619],
                           5793,
                           7046), 1305);
    assert_eq!(count_pairs(vec![1, 4, 2, 7], 2, 6), 6);
    assert_eq!(count_pairs(vec![9, 8, 4, 2, 1], 5, 14), 8);
    assert_eq!(count_pairs(vec![2, 2, 3, 3], 1, 100), 4);
}
