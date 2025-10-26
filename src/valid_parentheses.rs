impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c)
            } else {
                if stack.is_empty() {
                    return false;
                }
                if stack.pop() != match c {
                    ')' => '(',
                    '}' => '{',
                    ']' => '[',
                    _ => ' '
                }.into() {
                    return false;
                }
            }
        }

        stack.len() == 0
    }
}

pub struct Solution;