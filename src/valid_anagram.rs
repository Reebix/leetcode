impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut t_vec = t.chars().collect::<Vec<char>>();
        for c in s.chars() {
            if let Some(pos) = t_vec.iter().position(|&k| k == c) {
                t_vec.remove(pos);
            } else {
                return false;
            }
        }

        true
    }
}

pub struct Solution;