struct Solution;

// impl Solution {
//     pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
//         let mut ret: Vec<i32> = vec![];
//         for i in 0..arr.len() - k as usize + 1 {
//             ret.push(
//                 arr[i..i + k as usize]
//                     .iter()
//                     .map(|&n| (n - x).abs())
//                     .sum::<i32>(),
//             );
//         }
//         let min = *ret.iter().min().unwrap();
//         let po = ret.iter().position(|x| *x == min).unwrap();
//         arr[po..po + k as usize].to_vec()
//     }
// }

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (n, k) = (arr.len(), k as usize);
        let (mut l, mut r) = (0, n - k);
        while l < r {
            let mid = l + (r - l) / 2;
            if 2 * x > arr[mid] + arr[mid + k] {
                l = mid + 1
            } else {
                r = mid;
            }
        }
        arr[l..l + k].to_vec()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0685(){
        assert_eq!(Solution::find_closest_elements(vec![1,2,3,4,5],4,3),vec![1,2,3,4]);
    }
}
