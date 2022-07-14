//! 相似 RGB 颜色

pub fn similar_rgb(color: String) -> String {
    let mut max = i32::MIN;
    let mut result = String::new();
    let cr = i32::from_str_radix(&color[1..3], 16).unwrap();
    let cg = i32::from_str_radix(&color[3..5], 16).unwrap();
    let cb = i32::from_str_radix(&color[5..7], 16).unwrap();
    for r1 in 0..16 {
        for g1 in 0..16 {
            for b1 in 0..16 {
                let r = r1 * 16 + r1;
                let g = g1 * 16 + g1;
                let b = b1 * 16 + b1;
                let diff = -(cr - r).pow(2) - (cg - g).pow(2) - (cb - b).pow(2);
                if diff > max && (cr != r || cg != g || cb != b) {
                    max = diff;
                    result = format!("#{:02x}{:02x}{:02x}", r, g, b);
                }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(color: String) -> String) {
        assert_eq!(func(String::from("#09f166")), String::from("#11ee66"));
        assert_eq!(func(String::from("#4e3fe1")), String::from("#5544dd"));
    }
    test(similar_rgb);
}
