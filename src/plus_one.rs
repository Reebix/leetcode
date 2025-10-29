impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut index = digits.len() - 1;
        while carry != 0 {
            let res = digits[index] + carry;
            if res > 9 {
                digits[index] = 0;
                if index == 0 {
                    digits.insert(0, 1);
                    break;
                }
                carry = 1;
                index -= 1;
            } else {
                digits[index] = res;
                break;
            }
        }
        digits
    }
}

pub struct Solution;