use std::collections::HashMap;

// runtime seems random
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut chars: Vec<char>;

        for s in strs {
            chars = s.chars().collect::<Vec<char>>();
            chars.sort();

            map.entry(chars.iter().collect()).and_modify(|x| { (*x).push(s.clone()) }).or_insert(vec![s]);
        }


        map.into_values().collect()
    }
}

pub struct Solution;