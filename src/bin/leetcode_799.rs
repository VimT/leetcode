//! 香槟塔

pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut a = vec![vec![0.; 102]; 102];
    a[0][0] = poured as f64;
    let query_glass = query_glass as usize;
    let query_row = query_row as usize;
    for row in 0..=query_row {
        for i in 0..=query_glass {
            let q = (a[row][i] - 1.) / 2.;
            if q > 0. {
                a[row + 1][i] += q;
                a[row + 1][i + 1] += q;
            }
        }
    }
    (1f64).min(a[query_row][query_glass])
}

fn main() {
    assert_eq!(champagne_tower(1, 1, 1), 0.00000);
    assert_eq!(champagne_tower(2, 1, 1), 0.50000);
    assert_eq!(champagne_tower(100000009, 33, 17), 1.00000);
}
