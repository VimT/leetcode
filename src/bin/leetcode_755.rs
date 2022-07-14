//! 倒水

pub fn pour_water(mut heights: Vec<i32>, volume: i32, k: i32) -> Vec<i32> {
    for _ in 0..volume {
        let mut put = false;
        for d in [-1, 1] {
            let mut i = k;
            let mut best = k;
            while i + d >= 0 && i + d < heights.len() as i32 && heights[(i + d) as usize] <= heights[i as usize] {
                if heights[(i + d) as usize] < heights[i as usize] {
                    best = i + d;
                }
                i += d;
            }
            if best != k {
                heights[best as usize] += 1;
                put = true;
                break;
            }
        }
        if !put {
            heights[k as usize] += 1;
        }
    }
    heights
}

fn main() {
    fn test(func: fn(heights: Vec<i32>, volume: i32, k: i32) -> Vec<i32>) {
        assert_eq!(func(vec![2, 1, 1, 2, 1, 2, 2], 4, 3), vec![2, 2, 2, 3, 2, 2, 2]);
        assert_eq!(func(vec![1, 2, 3, 4], 2, 2), vec![2, 3, 3, 4]);
        assert_eq!(func(vec![3, 1, 3], 5, 1), vec![4, 4, 4]);
    }
    test(pour_water);
}
