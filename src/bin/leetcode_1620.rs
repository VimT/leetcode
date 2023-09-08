//! 网络信号最好的坐标

pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
    let xmax = towers.iter().map(|x| x[0]).max().unwrap();
    let ymax = towers.iter().map(|x| x[1]).max().unwrap();
    let mut result = vec![0, 0];
    let mut mx = 0;
    for x in 0..=xmax {
        for y in 0..=ymax {
            let mut value = 0;
            for tower in &towers {
                let (i, j, q) = (tower[0], tower[1], tower[2] as f64);
                let dis = (x - i) * (x - i) + (j - y) * (j - y);
                if dis <= radius * radius { value += (q / (1. + (dis as f64).sqrt())) as i32; }
            }
            if value > mx {
                result = vec![x, y];
                mx = value
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32>) {
        assert_eq!(func(vec![vec![44, 31, 4], vec![47, 27, 27], vec![7, 13, 0], vec![13, 21, 20], vec![50, 34, 18], vec![47, 44, 28]], 13), vec![47, 27]);
        assert_eq!(func(vec![vec![0, 1, 2], vec![2, 1, 2], vec![1, 0, 2], vec![1, 2, 2]], 1), vec![1, 1]);
        assert_eq!(func(vec![vec![50, 20, 31], vec![43, 36, 29]], 38), vec![50, 20]);
        assert_eq!(func(vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]], 2), vec![2, 1]);
        assert_eq!(func(vec![vec![23, 11, 21]], 9), vec![23, 11]);
        assert_eq!(func(vec![vec![1, 2, 13], vec![2, 1, 7], vec![0, 1, 9]], 2), vec![1, 2]);
    }
    test(best_coordinate);
}
