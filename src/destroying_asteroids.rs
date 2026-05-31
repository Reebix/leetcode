impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        // numbers get too big for some reason
        let mut mass = mass as i64;
        let mut asteroids = asteroids.iter().map(|a| *a as i64).collect::<Vec<_>>();

        asteroids.sort();

        for asteroid in asteroids {
            if mass < asteroid {
                return false;
            }

            mass += asteroid;
        }
        true
    }
}

pub struct Solution;
