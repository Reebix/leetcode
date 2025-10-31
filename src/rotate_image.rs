impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut res: Vec<Vec<i32>> = vec![vec![0i32; n]; n];

        for i in 0..n {
            for j in 0..n {
                res[j][n - 1 - i] = matrix[i][j];
            }
        }

        *matrix = res;
    }
}

pub struct Solution;