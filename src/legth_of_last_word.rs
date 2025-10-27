impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().split(" ").last().unwrap().len() as i32
    }
}

pub struct Solution;