impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut out: i32 = 0;
        while x != 0 {
            let r = x % 10;
            x -= r;
            x /= 10;
            let (o, overflow) = out.overflowing_mul(10);
            if overflow { return 0; }
            out = o;
            out += r;
        }
        out
    }
}

pub struct Solution;