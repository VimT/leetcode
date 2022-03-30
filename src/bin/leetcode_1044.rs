//! 最长重复子串

use std::collections::HashSet;

use rand::{Rng, thread_rng};

#[inline]
fn quick_pow(mut a: i64, mut b: i64, m: i64) -> i64 {
    a = a % m;
    let mut ans = 1;
    while b != 0 {
        if b & 1 == 1 {
            ans = ans * a % m;
            if a < 0 { a += m; }
        }
        a = a * a % m;
        if a < 0 {
            a += m;
        }
        b >>= 1;
    }
    ans
}


pub fn longest_dup_substring(s: String) -> String {
    let s = s.as_bytes();
    let mut sv = [0; 26];
    for &ch in s {
        sv[(ch - b'a') as usize] += 1;
    }
    let mut ok = false;
    for i in 0..26 {
        if sv[i] > 1 {
            ok = true;
            break;
        }
    }
    if !ok { return String::from(""); }
    let len = s.len();
    let mut left = 1;
    let mut right = len;
    let mut last = (0, 0);
    let mut rng = thread_rng();
    let base1 = rng.gen_range(26, 75);
    let base2 = rng.gen_range(26, 75);
    let mod1 = rng.gen_range(1e9 as i64 + 6, i32::MAX as i64);
    let mod2 = rng.gen_range(1e9 as i64 + 6, i32::MAX as i64);
    while left < right {
        let mid = (left + right) / 2;
        let mut set = HashSet::new();
        let mut has = false;
        let mut hash1 = 0;
        let mut hash2 = 0;
        let multi1 = quick_pow(base1, mid as i64, mod1);
        let multi2 = quick_pow(base2, mid as i64, mod2);
        for i in 0..mid {
            hash1 = (hash1 * base1 % mod1 + (s[i] - b'a') as i64) % mod1;
            hash2 = (hash2 * base2 % mod2 + (s[i] - b'a') as i64) % mod2;
        }
        set.insert(hash1);
        for i in mid..len {
            hash1 = (hash1 * base1 % mod1 - (multi1 * (s[i - mid] - b'a') as i64) % mod1 + (s[i] - b'a') as i64) % mod1;
            hash2 = (hash2 * base2 % mod2 - (multi2 * (s[i - mid] - b'a') as i64) % mod2 + (s[i] - b'a') as i64) % mod2;
            if hash1 < 0 { hash1 += mod1; }
            if set.contains(&(hash1)) {
                has = true;
                last = (i + 1 - mid, i);
                break;
            }
            set.insert(hash1);
        }
        if has {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    unsafe { String::from_utf8_unchecked(s[last.0..=last.1].to_vec()) }
}

#[inline]
fn cal_suffix_sa(s: &[u8]) -> (Vec<usize>, Vec<usize>) {
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

//计算height数组
//height[i]=suffix(sa[i-1])和 suffix(sa[i])的最长公共前缀，也就是排名相邻的两个后缀的最长公共前缀。
//这里借助h[i]=height[rank[i]]，也就是 suffix(i)和在它前一名的后缀的最长公共前缀
//用递推的方式计算。
fn get_height(s: &[u8], ranks: &Vec<usize>, suffix_array: &Vec<usize>) -> Vec<usize> {
    let len = s.len();
    let mut ll = 0;
    let mut height = vec![0; len];
    for i in 0..len {
        if ll > 0 {
            ll -= 1;
        }
        let rk = ranks[i];
        if rk >= 1 {
            let j = suffix_array[rk - 1];//排在ranks[i] - 1的后缀串j
            while i + ll < len && j + ll < len && s[i + ll] == s[j + ll] {
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

pub fn longest_dup_substring_sa(s: String) -> String {
    let s = s.as_bytes();
    let len = s.len();
    let (rank, sa) = cal_suffix_sa(s);
    let height = get_height(s, &rank, &sa);
    let mut p = 0;
    let mut ll = 0;
    for i in 0..len {
        if height[i] > ll {
            ll = height[i];
            p = sa[i];
        }
    }
    unsafe { String::from_utf8_unchecked(s[p..p + ll].to_vec()) }
}


fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("banana")), String::from("ana"));
        assert_eq!(func(String::from("nnpxouomcofdjuujloanjimymadkuepightrfodmauhrsy")), String::from("ma"));
        assert_eq!(func(String::from("aaaaa")), String::from("aaaa"));
        assert_eq!(func(String::from("abcd")), String::from(""));
    }
    test(longest_dup_substring);
    test(longest_dup_substring_sa);
}
