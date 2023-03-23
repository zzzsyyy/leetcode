struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ret: Vec<Vec<i32>> = vec![];
        let n = nums.len();
        for i in 0..n {
            let (mut j, mut l) = (i + 1, n - 1);
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            while j < l {
                match nums[i] + nums[j] + nums[l] {
                    t if t > 0 => l -= 1,
                    t if t < 0 => j += 1,
                    _ => {
                        ret.push(vec![nums[i], nums[j], nums[l]]);
                        j += 1;
                        while j < l && nums[j] == nums[j - 1] {
                            j += 1;
                        }
                        l -= 1;
                        while j < l && nums[l] == nums[l + 1] {
                            l -= 1;
                        }
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s0015() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vecnd![[-1, -1, 2], [-1, 0, 1]]
        );
    }
}
