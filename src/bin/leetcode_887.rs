//! 鸡蛋掉落


/// 动态规划: f(k,n) = max(f(k-1,x-1), f(k, n-x))
/// aux[e][f] 有e个鸡蛋，f个楼层时，需要移动的次数
fn super_egg_drop2(k: i32, n: i32) -> i32 {
    if n <= 2 || k == 1 { return n; }

    let egg_num = (k + 1) as usize;
    let floor = (n + 1) as usize;
    let mut aux = vec![vec![0; floor as usize]; egg_num];

    //初始化终止条件
    for i in 0..egg_num {
        aux[i][0] = 0;
        aux[i][1] = 1;
        aux[i][2] = 2;
    }
    for i in 3..floor {
        aux[1][i] = i;
    }

    for e in 2..egg_num {
        let mut x = 1;
        for f in 3..floor {
            //x取交汇处.(随着n的增大,x肯定也是增大的)
            while aux[e - 1][x - 1] < aux[e][f - x] {
                x += 1;
            }
            aux[e][f] = aux[e - 1][x - 1] + 1
        }
    }
    return aux[egg_num - 1][floor - 1] as i32;
}

/// 当我有egg个鸡蛋可以扔x次时，可以测出来多少层
/// f(egg, x) = max(f(egg, x-1), f(egg - 1, x-1))
fn super_egg_drop3(k: i32, n: i32) -> i32 {
    let egg_num = (k + 1) as usize;
    let step_num = (n + 1) as usize;
    let mut f = vec![vec![0; step_num]; egg_num];
    for i in 0..egg_num {
        f[i][0] = 0;
        f[i][1] = 1;
    }
    for i in 0..step_num {
        f[0][i] = 0;
        f[1][i] = i;
    }
    for x in 1..step_num {
        for egg in 1..egg_num {
            f[egg][x] = f[egg][x - 1] + f[egg - 1][x - 1] + 1;
            if f[egg][x] >= n as usize { return x as i32; }
        }
    }
    -1
}

/// 第三个方法，每次增加x，只用到了x-1行的数据，可以把二维表优化成线性表
fn super_egg_drop4(k: i32, n: i32) -> i32 {
    if n == 1 { return 1; }
    let egg_num = (k + 1) as usize;
    let mut f = vec![1; egg_num];
    f[0] = 0;
    for x in 1..n {
        for egg in (1..egg_num).rev() {
            f[egg] = f[egg] + f[egg - 1] + 1;
            if f[egg] >= n as usize { return (x + 1) as i32; }
        }
    }
    -1
}

fn main() {
    fn test(func: fn(k: i32, n: i32) -> i32) {
        assert_eq!(func(1, 2), 2);
        assert_eq!(func(2, 6), 3);
        assert_eq!(func(3, 14), 4);
    }
    test(super_egg_drop2);
    test(super_egg_drop3);
    test(super_egg_drop4);
}
