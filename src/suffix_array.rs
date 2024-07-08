pub fn cal_suffix_sa(s: &[u8]) -> (Vec<usize>, Vec<usize>) {
    let len = s.len();
    let mut bucket_cnt = 256;
    let mut bucket = vec![0; bucket_cnt];
    for &ch in s {
        bucket[ch as usize] += 1;
    }
    for i in 1..bucket_cnt {
        bucket[i] += bucket[i - 1];
    }
    let mut sa = vec![0; len];
    //根据桶排序的结果构造后缀数组
    for i in (0..len).rev() {
        bucket[s[i] as usize] -= 1;
        sa[bucket[s[i] as usize]] = i;
    }
    //借助后缀数组，计算排名数组
    let mut rank_no = 1;
    let mut ranks = vec![0; len];
    ranks[sa[0]] = 0;
    for i in 1..len {
        if s[sa[i - 1]] == s[sa[i]] {
            //相同字符，排名相同
            ranks[sa[i]] = rank_no - 1;
        } else {
            ranks[sa[i]] = rank_no;
            rank_no += 1;
        }
    }
    //2*len长度的前缀串
    let mut ll = 1;
    while ll <= len {
        //前缀串由两个len长的key1和key2拼接成。要对前缀串排序。
        //key2先排序。上一轮的suffix_array就已经确定key2的顺序了。
        //现在借助上一轮的suffix_array，按key2顺序取出对应的key1。
        let mut first_key_sorted: Vec<usize> = vec![0; len];
        let mut p = 0;
        for i in len - ll..len { //越界的key2，对应的key1一定排在最前面
            first_key_sorted[p] = i; //取对应的key1
            p += 1;
        }
        for i in 0..len {
            if sa[i] >= ll {
                //未越界的key2，对应的key1 等于 suffix_array[i] - len
                first_key_sorted[p] = sa[i] - ll;
                p += 1;
            }
        }
        //至此，first_key_sorted中放置了，已按key2排好序的key1。
        //在对key1的ranks进行桶排序
        bucket = vec![0; bucket_cnt];
        for i in 0..len {
            bucket[ranks[first_key_sorted[i]]] += 1;
        }
        for i in 1..bucket_cnt {
            bucket[i] += bucket[i - 1];
        }
        //根据桶排序的结果构造后缀数组
        for i in (0..len).rev() {
            bucket[ranks[first_key_sorted[i]]] -= 1;
            sa[bucket[ranks[first_key_sorted[i]]]] = first_key_sorted[i];
        }
        //借助suffix_array和上一轮的ranks，计算新的rank
        let mut new_rank = vec![0; len];
        new_rank[sa[0]] = 0;
        let mut rank_no = 1;
        for i in 1..len {
            //key1和key2的ranks都相等时，新排名相同。
            if ranks[sa[i - 1]] == ranks[sa[i]]
                && sa[i - 1].max(sa[i]) + ll < len
                && ranks[sa[i - 1] + ll] == ranks[sa[i] + ll] {
                new_rank[sa[i]] = rank_no - 1;
            } else {
                new_rank[sa[i]] = rank_no;
                rank_no += 1;
            }
        }
        ranks = new_rank;
        bucket_cnt = rank_no;
        if rank_no >= len {
            break;
        }

        ll *= 2;
    }
    (ranks, sa)
}

pub struct SuffixArray<'a> {
    pub s: &'a [u8],
    pub ranks: Vec<usize>,
    pub suffix_array: Vec<usize>,
}


impl<'a> SuffixArray<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        let (ranks, suffix_array) = cal_suffix_sa(s);
        Self { s, ranks, suffix_array }
    }

    pub fn get_height(&self) -> Vec<usize> {
        //计算height数组
        //height[i]=suffix(sa[i-1])和 suffix(sa[i])的最长公共前缀，也就是排名相邻的两个后缀的最长公共前缀。
        //这里借助h[i]=height[rank[i]]，也就是 suffix(i)和在它前一名的后缀的最长公共前缀
        //用递推的方式计算。
        let len = self.s.len();
        let mut ll = 0;
        let mut height = vec![0; len];
        for i in 0..len {
            if ll > 0 {
                ll -= 1;
            }
            let rk = self.ranks[i];
            if rk >= 1 {
                let j = self.suffix_array[rk - 1]; //排在ranks[i] - 1的后缀串j
                while i + ll < len && j + ll < len && self.s[i + ll] == self.s[j + ll] {
                    //计算后缀i与后缀j的LCP
                    ll += 1;
                }
                height[rk] = ll;
            } else {
                height[rk] = ll;
            }
        }
        height
    }

    /// 返回所有匹配的位置 ( unsorted )
    /// O(log(n) * len(word) + len(result))
    pub fn look_up(&self, word: &[u8]) -> Vec<usize> {
        // 找第一个 s.starts_with(word) 的位置
        let i = self.suffix_array.binary_search_by(|&x| {
            self.s[x..].cmp(word).then(std::cmp::Ordering::Greater)
        }).unwrap_err();
        // 从 i 开始，找第一个 !s.starts_with(word) 的位置
        let j = i + self.suffix_array[i..].binary_search_by(|&x| {
            if self.s[x..].starts_with(word) { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
        }).unwrap_err();
        self.suffix_array[i..j].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sa() {
        let s = "banana".as_bytes();
        let sa = SuffixArray::new(s);

        // suffix_array 表示后缀数组按字典序排序后的顺序
        // s[5..] a
        // s[3..] ana
        // s[1..] anana
        // s[0..] banana
        // s[4..] na
        // s[2..] nana
        assert_eq!(sa.suffix_array, vec![5, 3, 1, 0, 4, 2]);

        // ranks 表示后缀数组的排名
        // banana 排第 3 位
        //  anana 排第 2 位
        //   nana 排第 5 位
        //    ana 排第 1 位
        //     na 排第 4 位
        //      a 排第 0 位
        assert_eq!(sa.ranks, vec![3, 2, 5, 1, 4, 0]);

        // height 表示排名相邻的两个后缀的最长公共前缀
        let height = sa.get_height();
        // 0: a, nana
        // 1: ana, a
        // 3: anana, ana
        // 0: banana, anana
        // 0: na, banana
        // 2: nana, na
        assert_eq!(height, vec![0, 1, 3, 0, 0, 2]);

        let m = sa.look_up("ana".as_bytes());
        assert_eq!(m, vec![3, 1]);
    }
}
