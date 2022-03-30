//! 视频拼接

pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
    let time = time as usize;
    let mut m = vec![0; time as usize + 1];
    for clip in clips {
        if clip[0] as usize <= time {
            m[clip[0] as usize] = m[clip[0] as usize].max(clip[1] as usize);
        }
    }
    let mut cur = 0;
    let mut result = 0;
    while cur < time {
        let mut most = 0;
        for i in 0..=cur.min(time) {
            most = most.max(m[i]);
        }
        if most <= cur {
            return -1;
        }
        cur = most;
        result += 1;
    }
    result
}

fn main() {
    assert_eq!(video_stitching(vec![vec![5, 7], vec![1, 8], vec![0, 0], vec![2, 3], vec![4, 5], vec![0, 6], vec![5, 10], vec![7, 10]], 5), 1);
    assert_eq!(video_stitching(vec![vec![0, 4], vec![2, 8]], 5), 2);
    assert_eq!(video_stitching(vec![vec![0, 2], vec![4, 6], vec![8, 10], vec![1, 9], vec![1, 5], vec![5, 9]], 10), 3);
    assert_eq!(video_stitching(vec![vec![0, 1], vec![1, 2]], 5), -1);
    assert_eq!(video_stitching(vec![vec![0, 1], vec![6, 8], vec![0, 2], vec![5, 6], vec![0, 4], vec![0, 3], vec![6, 7], vec![1, 3], vec![4, 7], vec![1, 4], vec![2, 5], vec![2, 6], vec![3, 4], vec![4, 5], vec![5, 7], vec![6, 9]], 9), 3);
}
