impl Solution {
    pub fn has_same_digits(mut s: String) -> bool {
        let mut out = "".to_string();
        while s.len() != 2 {
            out.clear();
            for i in 0..s.len() - 1 {
                out.push(((s.chars().collect::<Vec<_>>()[i].to_digit(10).unwrap() + s.chars().collect::<Vec<_>>()[i + 1].to_digit(10).unwrap()) % 10).to_string().chars().next().unwrap());
            }
            s = out.to_string();
        }

        s.chars().collect::<Vec<_>>()[0] == s.chars().collect::<Vec<_>>()[1]
    }
}

pub struct Solution;