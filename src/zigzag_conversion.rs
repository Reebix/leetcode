impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut lines: Vec<String> = Vec::with_capacity(num_rows as usize);
        for i in 0..num_rows {
            lines.push(String::new());
        }
        let mut down = true;
        let mut i = 0;
        s.chars().for_each(|c| {
            lines[i].push(c);
            if down {
                i += 1;
            } else {
                i -= 1;
            }
            if i == (num_rows - 1) as usize {
                down = false;
            }
            if i == 0 {
                down = true;
            }
        });
        println!("{:?}", lines);
        let out = lines.join("");
        out
    }
}

pub struct Solution;