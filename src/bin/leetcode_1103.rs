//! 分糖果 II

pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
    let mut result = vec![0; num_people as usize];
    let mut lun = 0;
    while (lun * num_people + 1) * (lun * num_people) / 2 <= candies {
        lun += 1;
    }
    lun -= 1;
    for i in 0..num_people {
        result[i as usize] = ((lun - 1) * num_people + 2 * (i + 1)) * lun / 2;
    }
    candies -= (lun * num_people + 1) * (lun * num_people) / 2;
    let mut i = 0;
    while candies > 0 {
        let can = lun * num_people + (i + 1);
        result[i as usize] += can.min(candies);
        candies -= can;
        i += 1;
    }
    result
}

fn main() {
    fn test(func: fn(candies: i32, num_people: i32) -> Vec<i32>) {
        assert_eq!(func(60, 4), vec![15, 18, 15, 12]);
        assert_eq!(func(7, 4), vec![1, 2, 3, 1]);
        assert_eq!(func(10, 3), vec![5, 2, 3]);
    }
    test(distribute_candies);
}
