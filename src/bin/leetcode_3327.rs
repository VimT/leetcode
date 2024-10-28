//! 判断 DFS 字符串是否是回文串

pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
    let n = parent.len();
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 1..n { g[parent[i] as usize].push(i); }
    // dfs_str 是后序遍历整棵树得到的字符串
    let mut dfs_str: Vec<u8> = vec![b'\0'; n];
    // nodes[i] 表示子树 i 的后序遍历的开始时间戳和结束时间戳+1（左闭右开区间）
    let mut nodes: Vec<[usize; 2]> = vec![[0, 0]; n];
    let mut time = 0;
    // 将字符串转为char数组以便索引
    let s = s.as_bytes();

    // 后序遍历函数
    fn dfs(g: &Vec<Vec<usize>>, s: &[u8], x: usize, dfs_str: &mut Vec<u8>, nodes: &mut Vec<[usize; 2]>, time: &mut usize) {
        nodes[x][0] = *time;
        for &y in &g[x] {
            dfs(g, s, y, dfs_str, nodes, time);
        }
        dfs_str[*time] = s[x];  // 后序遍历
        *time += 1;
        nodes[x][1] = *time;
    }

    dfs(&g, &s, 0, &mut dfs_str, &mut nodes, &mut time);

    // Manacher 模板
    // 将 dfs_str 改造为 t，这样就不需要讨论 n 的奇偶性，因为新串 t 的每个回文子串都是奇回文串（都有回文中心）
    // dfs_str 和 t 的下标转换关系：
    // (dfs_str_i+1)*2 = ti
    // ti/2-1 = dfs_str_i
    // ti 为偶数，对应奇回文串（从 2 开始）
    // ti 为奇数，对应偶回文串（从 3 开始）
    let mut t = vec![b'^'];
    for &c in dfs_str.iter() {
        t.push(b'#');
        t.push(c);
    }
    t.push(b'#');
    t.push(b'$');

    // 定义一个奇回文串的回文半径=(长度+1)/2，即保留回文中心，去掉一侧后的剩余字符串的长度
    // half_len[i] 表示在 t 上的以 t[i] 为回文中心的最长回文子串的回文半径
    // 即 [i-half_len[i]+1,i+half_len[i]-1] 是 t 上的一个回文子串
    let mut half_len = vec![0; t.len() - 2];
    half_len[1] = 1;

    // box_r 表示当前右边界下标最大的回文子串的右边界下标+1
    // box_m 为该回文子串的中心位置，二者的关系为 r=mid+half_len[mid]
    let (mut box_m, mut box_r) = (0, 0);

    for i in 2..half_len.len() {
        let mut hl = 1;
        if i < box_r {
            // 记 i 关于 box_m 的对称位置 i'=box_m*2-i
            // 若以 i' 为中心的最长回文子串范围超出了以 box_m 为中心的回文串的范围（即 i+half_len[i'] >= box_r）
            // 则 half_len[i] 应先初始化为已知的回文半径 box_r-i，然后再继续暴力匹配
            // 否则 half_len[i] 与 half_len[i'] 相等
            hl = half_len[box_m * 2 - i].min(box_r - i);
        }

        // 暴力扩展
        // 算法的复杂度取决于这部分执行的次数
        // 由于扩展之后 box_r 必然会更新（右移），且扩展的的次数就是 box_r 右移的次数
        // 因此算法的复杂度 = O(len(t)) = O(n)
        while t[i - hl] == t[i + hl] {
            hl += 1;
        }
        half_len[i] = hl;
        if i + hl > box_r {
            box_m = i;
            box_r = i + hl;
        }
    }

    // t 中回文子串的长度为 hl*2-1
    // 由于其中 # 的数量总是比字母的数量多 1
    // 因此其在 dfs_str 中对应的回文子串的长度为 hl-1
    // 这一结论可用在 is_palindrome 中

    // 判断左闭右开区间 [l,r) 是否为回文串  0<=l<r<=n
    // 根据下标转换关系得到 dfs_str 的 [l,r) 子串在 t 中对应的回文中心下标为 l+r+1
    // 需要满足 half_len[l + r + 1] - 1 >= r - l，即 half_len[l + r + 1] > r - l
    nodes.iter().map(|&[l, r]| half_len[l + r + 1] > r - l).collect()
}

fn main() {
    fn test(func: fn(parent: Vec<i32>, s: String) -> Vec<bool>) {
        assert_eq!(func(vec![-1, 0, 0, 1, 1, 2], String::from("aababa")), vec![true, true, false, true, true, true]);
        assert_eq!(func(vec![-1, 0, 0, 0, 0], String::from("aabcb")), vec![true, true, true, true, true]);
    }
    test(find_answer);
}
