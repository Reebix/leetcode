use std::collections::HashSet;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut ignored_special: HashSet<char> = HashSet::new();
        let mut valid_special: HashSet<char> = HashSet::new();
        let mut current_special: HashSet<char> = HashSet::new();
        let mut found_uppercase: HashSet<char> = HashSet::new();

        for c in word.chars() {
            if c.is_uppercase() {
                if !ignored_special.contains(&c.to_ascii_lowercase()) && current_special.contains(&c.to_ascii_lowercase()) {
                    valid_special.insert(c.to_ascii_lowercase());
                }
                found_uppercase.insert(c);
            } else {
                current_special.insert(c);

                if found_uppercase.contains(&c.to_ascii_uppercase()) {
                    ignored_special.insert(c);
                }
            }
        }

        println!("{:?} {:?}", valid_special, ignored_special);

        valid_special.retain(|c| !ignored_special.contains(c));
        valid_special.len() as i32
    }
}

pub struct Solution;