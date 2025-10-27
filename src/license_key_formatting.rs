impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        if k >= s.len() as i32 { return s.to_uppercase().replace("-", ""); }

        let mut out = "".to_owned();
        let chars = s.to_uppercase().replace("-", "").chars().collect::<Vec<_>>();
        let first = chars.len() % k as usize;
        for i in 0..first {
            out += &chars[i].to_string();
        }
        if first != 0 {
            out += "-";
        }
        let mut c = 1;
        for i in first..chars.len() - 1 {
            out += &chars[i].to_string();
            if c % k as usize == 0 {
                out += "-";
                c = 0;
            }
            c += 1;
        }
        out += &chars.last().unwrap().to_string();

        out.to_string()
    }
}

pub struct Solution;