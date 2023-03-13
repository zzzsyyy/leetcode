struct Solution;

impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let n1 = energy.iter().sum::<i32>() + 1;
        let ret1 = if n1 - initial_energy > 0 {
            n1 - initial_energy
        } else {
            0
        };
        let mut ret2 = 0;
        let mut tmp = initial_experience;
        for &n in experience.iter() {
            let a = n - tmp + 1;
            if a > 0 {
                ret2 += a;
                tmp += a;
            }
            tmp += n;
        }
        ret1 + ret2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s2383() {
        assert_eq!(
            Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
            8
        );
        assert_eq!(
            Solution::min_number_of_hours(1, 1, vec![1, 1, 1, 1], vec![1, 1, 1, 50]),
            51
        );
    }
}
