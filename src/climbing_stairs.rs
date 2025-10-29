impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }
        let mut a = 1;
        let mut b = 2;

        for _ in 2..n {
            (b, a) = (b + a, b);
        }
        b
    }
}

pub struct Solution;