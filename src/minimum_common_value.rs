impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i1 = 0;
        let mut i2 = 0;

        while i1 < nums1.len() && i2 < nums2.len() {
            let l = nums1[i1];
            let r = nums2[i2];

            if l == r {
                return l;
            }

            if l > r {
                i2 += 1;
            } else {
                i1 += 1;
            }
        }

        -1
    }
}

pub struct Solution;
